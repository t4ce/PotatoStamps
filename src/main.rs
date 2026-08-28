//! Potato Stamps: an immutable Picasso scene document rendered through the
//! TRUEOS Picasso retained rendering path.
//!
//! The application intentionally has no Blueprint registration.  It becomes
//! runnable only when a future Blueprint explicitly packages it.
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt;
use potato_stamps::scene::{
    self, COLOR_TEXTURE_BYTES, COLOR_TEXTURE_NAME, DOCUMENT_BYTES, DOCUMENT_NAME, INDEX_COUNT,
    Scene, VERTEX_COUNT,
};
use trueos::ui4_scene::{Damage, Error as Ui4Error, Frame};
use trueos::vgpu::{
    BUFFER_USAGE_INDEX, BUFFER_USAGE_MAP_WRITE, BUFFER_USAGE_VERTEX, Buffer, Capabilities, Device,
    Queue, QueueClass, RetainedFrameSubmit, RetainedMesh, RetainedMeshDescriptor,
    RetainedTransformSeed,
};
use trueos::{
    clock,
    logl::{self, level},
    vsys,
};
use trueos_picasso::Picasso;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

// Keep the package directly `cargo check`-able as a thin no_std Blueprint.
// TRUEOS's packer detects these same declarations when it prepares a future
// Blueprint package.
#[global_allocator]
static ALLOCATOR: trueos::TrueosAllocator = trueos::TrueosAllocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    trueos::panic_abort("Potato Stamps panic\n")
}

/// Picasso-decoded geometry and palette remain immutable after `open` hands
/// them to the retained carrier.
struct PotatoStamps {
    _picasso: Picasso,
    frame: Frame,
    device: Device,
    queue: Queue,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    retained_mesh: RetainedMesh,
    palette: trueos::vmedia::RetainedTexture,
    timeline: u64,
}

impl PotatoStamps {
    fn open() -> Result<Self, DemoError> {
        let picasso = Picasso::new().map_err(DemoError::Picasso)?;
        picasso
            .put_embedded_asset(DOCUMENT_NAME, DOCUMENT_BYTES)
            .map_err(DemoError::Picasso)?;
        picasso
            .put_embedded_asset(COLOR_TEXTURE_NAME, COLOR_TEXTURE_BYTES)
            .map_err(DemoError::Picasso)?;
        let stored = picasso
            .embedded_asset(DOCUMENT_NAME)
            .map_err(DemoError::Picasso)?
            .ok_or(DemoError::MissingAsset)?;
        let stored_palette = picasso
            .embedded_asset(COLOR_TEXTURE_NAME)
            .map_err(DemoError::Picasso)?
            .ok_or(DemoError::MissingPalette)?;
        let scene = Scene::decode(&stored).map_err(DemoError::Scene)?;

        let frame = Frame::open_streaming(96, 72, WIDTH, HEIGHT)
            .map_err(|error| DemoError::Ui4("frame-open", error))?;
        let device = Device::open(Capabilities::DEFAULT.union(Capabilities::PRESENT))
            .map_err(|code| DemoError::Vgpu("device-open", code))?;
        let queue = device
            .create_queue(QueueClass::Render)
            .map_err(|code| DemoError::Vgpu("queue-create", code))?;
        let retained_vertices = retained_vertex_bytes(&scene);
        let retained_indices = retained_index_bytes(&scene);
        let vertex_buffer = device
            .create_buffer(
                retained_vertices.len(),
                BUFFER_USAGE_MAP_WRITE | BUFFER_USAGE_VERTEX,
            )
            .map_err(|code| DemoError::Vgpu("vertex-buffer-create", code))?;
        let index_buffer = device
            .create_buffer(
                retained_indices.len(),
                BUFFER_USAGE_MAP_WRITE | BUFFER_USAGE_INDEX,
            )
            .map_err(|code| DemoError::Vgpu("index-buffer-create", code))?;

        write_exact(device, vertex_buffer, &retained_vertices)
            .map_err(|code| DemoError::Vgpu("vertex-upload", code))?;
        write_exact(device, index_buffer, &retained_indices)
            .map_err(|code| DemoError::Vgpu("index-upload", code))?;
        let retained_mesh = device
            .create_retained_mesh(
                vertex_buffer,
                index_buffer,
                RetainedMeshDescriptor {
                    vertex_count: VERTEX_COUNT as u32,
                    index_count: INDEX_COUNT as u32,
                    vertex_layout: trueos::vgpu::RETAINED_VERTEX_LAYOUT_POS_NORMAL_UV,
                    ..RetainedMeshDescriptor::default()
                },
            )
            .map_err(|code| DemoError::Vgpu("retained-mesh-create", code))?;
        let palette = trueos::async_fs::block_on(trueos::vmedia::decode_retained_asset(
            device,
            COLOR_TEXTURE_NAME,
            &stored_palette,
        ))
        .map_err(|code| DemoError::Vgpu("palette-decode", code))?;

        logl::log(
            level::INFO,
            format_args!(
                "PotatoStamps: Picasso readback accepted document={} document_bytes={} palette={} palette_bytes={} vertices={} opaque_triangles={}",
                DOCUMENT_NAME,
                stored.len(),
                COLOR_TEXTURE_NAME,
                stored_palette.len(),
                VERTEX_COUNT,
                INDEX_COUNT / 3,
            ),
        );
        Ok(Self {
            _picasso: picasso,
            frame,
            device,
            queue,
            vertex_buffer,
            index_buffer,
            retained_mesh,
            palette,
            timeline: 0,
        })
    }

    fn render_frame(&mut self) -> Result<(), DemoError> {
        let width = self.frame.width();
        let height = self.frame.height();
        self.frame
            .begin_gpu_frame()
            .map_err(|error| DemoError::Ui4("frame-begin", error))?;
        let surface = self
            .device
            .acquire_ui4_surface(self.frame.window_id())
            .map_err(|code| DemoError::Vgpu("surface-acquire", code))?;
        let point = self
            .device
            .submit_retained_frame(
                self.queue,
                surface,
                self.retained_mesh,
                self.vertex_buffer,
                self.index_buffer,
                RetainedFrameSubmit {
                    clear_rgba8_srgb: u32::from_le_bytes([21, 25, 32, 255]),
                    base_color_texture: self.palette.id().raw(),
                    seed_count: 1,
                    seeds: identity_retained_seeds(),
                    ..RetainedFrameSubmit::default()
                },
            )
            .map_err(|code| DemoError::Vgpu("retained-frame-submit", code))?;
        self.device
            .wait(self.queue, point.value)
            .map_err(|code| DemoError::Vgpu("timeline-wait", code))?;
        self.frame
            .publish(Damage::full(width, height))
            .map_err(|error| DemoError::Ui4("frame-publish", error))?;
        self.timeline = point.value;
        Ok(())
    }
}

fn retained_vertex_bytes(scene: &Scene) -> Vec<u8> {
    let mut retained = Vec::with_capacity(VERTEX_COUNT * 24);
    for position in &scene.positions {
        retained.extend_from_slice(&position.x.to_le_bytes());
        retained.extend_from_slice(&position.y.to_le_bytes());
        retained.extend_from_slice(&position.z.to_le_bytes());
        // The current retained textured carrier forwards normal.xy into its
        // sampler proof. The authored document makes this palette selector
        // explicit; each triangle uses one constant texel coordinate.
        retained.extend_from_slice(&position.texture_u.to_le_bytes());
        retained.extend_from_slice(&position.texture_v.to_le_bytes());
        retained.extend_from_slice(&1.0f32.to_le_bytes());
        retained.extend_from_slice(&0.0f32.to_le_bytes());
        retained.extend_from_slice(&0.0f32.to_le_bytes());
    }
    retained
}

fn retained_index_bytes(scene: &Scene) -> Vec<u8> {
    let mut retained = Vec::with_capacity(INDEX_COUNT * core::mem::size_of::<u32>());
    for index in scene.indices {
        retained.extend_from_slice(&index.to_le_bytes());
    }
    retained
}

fn identity_retained_seeds() -> [RetainedTransformSeed; trueos::vgpu::MAX_RETAINED_TRANSFORM_SEEDS]
{
    let mut seeds = [RetainedTransformSeed::default(); trueos::vgpu::MAX_RETAINED_TRANSFORM_SEEDS];
    seeds[0] = RetainedTransformSeed {
        translation: [0.0, 0.0, 0.0],
        scale: [1.0, 1.0, 1.0],
        rotation: [0.0, 0.0, 0.0, 1.0],
        local_radius: 2.0,
        previous_translation: [0.0, 0.0, 0.0],
        draw_group: 0,
        flags: 0,
    };
    seeds
}

fn write_exact(device: Device, buffer: Buffer, bytes: &[u8]) -> Result<(), i32> {
    let written = device.write_buffer(buffer, 0, bytes)?;
    (written == bytes.len())
        .then_some(())
        .ok_or(trueos::vgpu::ERR_IO)
}

#[derive(Debug)]
enum DemoError {
    MissingAsset,
    MissingPalette,
    Picasso(trueos_picasso::PicassoError),
    Scene(scene::SceneError),
    Ui4(&'static str, Ui4Error),
    Vgpu(&'static str, i32),
}

impl fmt::Display for DemoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingAsset => f.write_str("Picasso did not return the seeded document"),
            Self::MissingPalette => f.write_str("Picasso did not return the seeded palette"),
            Self::Picasso(error) => write!(f, "Picasso storage error: {error}"),
            Self::Scene(error) => write!(f, "scene document error: {error}"),
            Self::Ui4(stage, error) => write!(f, "UI4 {stage} failed: {error:?}"),
            Self::Vgpu(stage, error) => write!(f, "vGPU {stage} failed: {error}"),
        }
    }
}

fn main() {
    if let Err(error) = run() {
        logl::log(
            level::ERROR,
            format_args!("PotatoStamps: fatal error: {error}"),
        );
        if !trueos::vshell::shutdown_current_blueprint(
            "Potato Stamps terminated after a fatal error",
        ) {
            logl::log(
                level::ERROR,
                "PotatoStamps: could not request Blueprint shutdown",
            );
        }
    }
}

fn run() -> Result<(), DemoError> {
    let mut demo = PotatoStamps::open()?;
    demo.render_frame()?;
    logl::log(
        level::INFO,
        format_args!(
            "PotatoStamps: retained triangle frame submitted and retired timeline={}",
            demo.timeline
        ),
    );
    let start = clock::monotonic_millis();
    loop {
        vsys::poll_once();
        demo.render_frame()?;
        // The immutable data remains GPU resident; this sleep only paces UI4.
        vsys::sleep_ms(16);
        let _elapsed = clock::monotonic_millis().saturating_sub(start);
    }
}
