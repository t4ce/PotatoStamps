//! Potato Stamps: an immutable Picasso scene document rendered through the
//! TRUEOS immediate primitive path.
//!
//! The application intentionally has no Blueprint registration.  It becomes
//! runnable only when a future Blueprint explicitly packages it.
#![no_std]

use core::fmt;
use potato_stamps::scene::{self, DOCUMENT_BYTES, DOCUMENT_NAME, Scene, StampMode, VERTEX_COUNT};
use trueos::ui4_scene::{Damage, Error as Ui4Error, Frame};
use trueos::vgpu::{
    BUFFER_USAGE_INDEX, BUFFER_USAGE_MAP_WRITE, BUFFER_USAGE_VERTEX, Buffer, Capabilities, Device,
    IndexedBatchDrawV2, IndexedDrawBatchV2, Queue, QueueClass, RenderPipeline,
    SHADER_PACKAGE_CLIP_POSITION3_IMMEDIATE_RGBA_FNV1A64, ShaderModule,
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

/// The authority signal affects the selection descriptor only. Both GPU
/// buffers stay immutable after `open` has uploaded the Picasso-decoded scene.
struct PotatoStamps {
    _picasso: Picasso,
    frame: Frame,
    device: Device,
    queue: Queue,
    _shader: ShaderModule,
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    mode: StampMode,
    previous_keys: u8,
    timeline: u64,
}

impl PotatoStamps {
    fn open() -> Result<Self, DemoError> {
        let picasso = Picasso::new().map_err(DemoError::Picasso)?;
        picasso
            .put_embedded_asset(DOCUMENT_NAME, DOCUMENT_BYTES)
            .map_err(DemoError::Picasso)?;
        let stored = picasso
            .embedded_asset(DOCUMENT_NAME)
            .map_err(DemoError::Picasso)?
            .ok_or(DemoError::MissingAsset)?;
        let scene = Scene::decode(&stored).map_err(DemoError::Scene)?;

        let frame = Frame::open_streaming(96, 72, WIDTH, HEIGHT)
            .map_err(|error| DemoError::Ui4("frame-open", error))?;
        let device = Device::open(Capabilities::DEFAULT.union(Capabilities::PRESENT))
            .map_err(|code| DemoError::Vgpu("device-open", code))?;
        let queue = device
            .create_queue(QueueClass::Render)
            .map_err(|code| DemoError::Vgpu("queue-create", code))?;
        let shader = device
            .create_shader_module(SHADER_PACKAGE_CLIP_POSITION3_IMMEDIATE_RGBA_FNV1A64)
            .map_err(|code| DemoError::Vgpu("shader-create", code))?;
        let pipeline = device
            .create_render_pipeline(shader, core::mem::size_of::<scene::Position>() as u32, 0)
            .map_err(|code| DemoError::Vgpu("pipeline-create", code))?;
        let vertex_buffer = device
            .create_buffer(
                scene.position_bytes().len(),
                BUFFER_USAGE_MAP_WRITE | BUFFER_USAGE_VERTEX,
            )
            .map_err(|code| DemoError::Vgpu("vertex-buffer-create", code))?;
        let index_buffer = device
            .create_buffer(
                scene.index_bytes().len(),
                BUFFER_USAGE_MAP_WRITE | BUFFER_USAGE_INDEX,
            )
            .map_err(|code| DemoError::Vgpu("index-buffer-create", code))?;

        write_exact(device, vertex_buffer, scene.position_bytes())
            .map_err(|code| DemoError::Vgpu("vertex-upload", code))?;
        write_exact(device, index_buffer, scene.index_bytes())
            .map_err(|code| DemoError::Vgpu("index-upload", code))?;

        logl::log(
            level::INFO,
            format_args!(
                "PotatoStamps: Picasso readback accepted document={} source_bytes={} vertices={} immutable_index_words={}",
                DOCUMENT_NAME,
                stored.len(),
                VERTEX_COUNT,
                scene.indices.len(),
            ),
        );
        Ok(Self {
            _picasso: picasso,
            frame,
            device,
            queue,
            _shader: shader,
            pipeline,
            vertex_buffer,
            index_buffer,
            mode: StampMode::Triangle,
            previous_keys: 0,
            timeline: 0,
        })
    }

    fn render_frame(&mut self) -> Result<(), DemoError> {
        self.service_authority_signal()?;
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
            .submit_ui4_indexed_batch_v2(
                self.queue,
                surface,
                self.pipeline,
                self.vertex_buffer,
                self.index_buffer,
                self.batch_for_authorized_mode(),
            )
            .map_err(|code| DemoError::Vgpu("indexed-batch-submit", code))?;
        self.device
            .wait(self.queue, point.value)
            .map_err(|code| DemoError::Vgpu("timeline-wait", code))?;
        self.frame
            .publish(Damage::full(width, height))
            .map_err(|error| DemoError::Ui4("frame-publish", error))?;
        self.timeline = point.value;
        Ok(())
    }

    /// 1/2/3 are CPU-originating authorization signals. They choose an
    /// already-seeded index range and topology; no GPU buffer is rewritten.
    fn service_authority_signal(&mut self) -> Result<(), DemoError> {
        let state = self
            .frame
            .keyboard_state()
            .map_err(|error| DemoError::Ui4("authority-keyboard", error))?;
        let keys = state.map_or(0, |keyboard| {
            [0x02u8, 0x03, 0x04]
                .into_iter()
                .enumerate()
                .fold(0u8, |bits, (slot, key)| {
                    bits | ((keyboard.is_down(key) as u8) << slot)
                })
        });
        let pressed = keys & !self.previous_keys;
        self.previous_keys = keys;
        if pressed == 0 {
            return Ok(());
        }
        let key = if pressed & 1 != 0 {
            0x02
        } else if pressed & 2 != 0 {
            0x03
        } else {
            0x04
        };
        if let Some(mode) = StampMode::from_authority_key(key) {
            self.mode = mode;
            let selection = mode.selection();
            logl::log(
                level::INFO,
                format_args!(
                    "PotatoStamps: authority mode={} topology={} first_index={} index_count={} vertex_uploads=1 index_uploads=1",
                    mode.label(),
                    selection.topology,
                    selection.first_index,
                    selection.index_count,
                ),
            );
        }
        Ok(())
    }

    fn batch_for_authorized_mode(&self) -> IndexedDrawBatchV2 {
        let selection = self.mode.selection();
        IndexedDrawBatchV2 {
            clear_rgba8_srgb: u32::from_le_bytes([21, 25, 32, 255]),
            draw_count: 4,
            draws: core::array::from_fn(|tile| IndexedBatchDrawV2 {
                index_count: selection.index_count,
                first_index: selection.first_index,
                base_vertex: (tile * 3) as i32,
                rgba8_srgb: [
                    u32::from_le_bytes([238, 174, 65, 255]),
                    u32::from_le_bytes([245, 199, 91, 255]),
                    u32::from_le_bytes([219, 139, 46, 255]),
                    u32::from_le_bytes([255, 211, 111, 255]),
                ][tile],
                topology: selection.topology,
                reserved: 0,
            }),
            ..IndexedDrawBatchV2::default()
        }
    }
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
    Picasso(trueos_picasso::PicassoError),
    Scene(scene::SceneError),
    Ui4(&'static str, Ui4Error),
    Vgpu(&'static str, i32),
}

impl fmt::Display for DemoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingAsset => f.write_str("Picasso did not return the seeded document"),
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
    let start = clock::monotonic_millis();
    loop {
        vsys::poll_once();
        demo.render_frame()?;
        // The immutable data remains GPU resident; this sleep only paces UI4.
        vsys::sleep_ms(16);
        let _elapsed = clock::monotonic_millis().saturating_sub(start);
    }
}
