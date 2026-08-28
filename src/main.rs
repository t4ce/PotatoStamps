//! Potato Stamps: an immutable Picasso scene document rendered through the
//! TRUEOS Picasso immediate indexed rendering path.
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt;
use potato_stamps::scene::{
    self, COLOR_TEXTURE_BYTES, COLOR_TEXTURE_NAME, DOCUMENT_BYTES, DOCUMENT_NAME,
    EXECUTION_INDEX_COUNT, ExecutionIndexCatalogue, PrimitiveMode, STAMP_COUNT, Scene,
    VERTEX_COUNT, decode_palette_rgba,
};
use trueos::ui4_scene::{Damage, Error as Ui4Error, Frame};
use trueos::vgpu::{
    BUFFER_USAGE_INDEX, BUFFER_USAGE_MAP_WRITE, BUFFER_USAGE_VERTEX, Buffer, Capabilities, Device,
    Queue, QueueClass, RenderPipeline, SHADER_PACKAGE_CLIP_POSITION3_IMMEDIATE_RGBA_FNV1A64,
    ShaderModule,
};
use trueos::{
    clock,
    logl::{self, level},
    vsys,
};
use trueos_picasso::Picasso;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;
const CLEAR_RGBA8_SRGB: u32 = u32::from_le_bytes([21, 25, 32, 255]);

// Keep the registered package directly `cargo check`-able as a thin no_std
// Blueprint. TRUEOS's packer detects these declarations for the startup image.
#[global_allocator]
static ALLOCATOR: trueos::TrueosAllocator = trueos::TrueosAllocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    trueos::panic_abort("Potato Stamps panic\n")
}

/// Picasso-decoded geometry, topology catalogue, and colors are immutable
/// after `open` uploads the two execution buffers.
struct PotatoStamps {
    _picasso: Picasso,
    frame: Frame,
    device: Device,
    queue: Queue,
    _shader: ShaderModule,
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    catalogue: ExecutionIndexCatalogue,
    colors: [u32; STAMP_COUNT],
    selected_mode: usize,
    number_keys: u8,
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
        let shader = device
            .create_shader_module(SHADER_PACKAGE_CLIP_POSITION3_IMMEDIATE_RGBA_FNV1A64)
            .map_err(|code| DemoError::Vgpu("shader-create", code))?;
        let pipeline = device
            .create_render_pipeline(shader, 12, 0)
            .map_err(|code| DemoError::Vgpu("pipeline-create", code))?;
        let execution_vertices = execution_vertex_bytes(&scene);
        let catalogue = scene.execution_index_catalogue();
        let execution_indices = execution_index_bytes(&catalogue);
        let vertex_buffer = device
            .create_buffer(
                execution_vertices.len(),
                BUFFER_USAGE_MAP_WRITE | BUFFER_USAGE_VERTEX,
            )
            .map_err(|code| DemoError::Vgpu("vertex-buffer-create", code))?;
        let index_buffer = device
            .create_buffer(
                execution_indices.len(),
                BUFFER_USAGE_MAP_WRITE | BUFFER_USAGE_INDEX,
            )
            .map_err(|code| DemoError::Vgpu("index-buffer-create", code))?;

        write_exact(device, vertex_buffer, &execution_vertices)
            .map_err(|code| DemoError::Vgpu("vertex-upload", code))?;
        write_exact(device, index_buffer, &execution_indices)
            .map_err(|code| DemoError::Vgpu("index-upload", code))?;
        let colors = decode_palette_rgba(&stored_palette).ok_or(DemoError::InvalidPalette)?;

        logl::log(
            level::INFO,
            format_args!(
                "PotatoStamps: Picasso readback accepted document={} document_bytes={} palette={} palette_bytes={} vertices={} execution_indices={} native_modes={}",
                DOCUMENT_NAME,
                stored.len(),
                COLOR_TEXTURE_NAME,
                stored_palette.len(),
                VERTEX_COUNT,
                EXECUTION_INDEX_COUNT,
                scene::PRIMITIVE_MODE_COUNT,
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
            catalogue,
            colors,
            selected_mode: 3,
            number_keys: 0,
            timeline: 0,
        })
    }

    fn render_frame(&mut self) -> Result<(), DemoError> {
        self.service_mode_hotkeys()?;
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
                self.catalogue.draw_batch(
                    PrimitiveMode::ALL[self.selected_mode],
                    self.colors,
                    CLEAR_RGBA8_SRGB,
                ),
            )
            .map_err(|code| DemoError::Vgpu("indexed-batch-v2-submit", code))?;
        self.device
            .wait(self.queue, point.value)
            .map_err(|code| DemoError::Vgpu("timeline-wait", code))?;
        self.frame
            .publish(Damage::full(width, height))
            .map_err(|error| DemoError::Ui4("frame-publish", error))?;
        self.timeline = point.value;
        Ok(())
    }

    fn service_mode_hotkeys(&mut self) -> Result<(), DemoError> {
        let state = self
            .frame
            .keyboard_state()
            .map_err(|error| DemoError::Ui4("mode-hotkeys", error))?;
        let current = state.map_or(0, |keyboard| {
            let mut bits = 0u8;
            for slot in 0..scene::PRIMITIVE_MODE_COUNT {
                if keyboard.is_down(0x1e + slot as u8) {
                    bits |= 1 << slot;
                }
            }
            bits
        });
        let pressed = current & !self.number_keys;
        self.number_keys = current;
        if pressed != 0 {
            self.selected_mode = pressed.trailing_zeros() as usize;
            logl::log(
                level::INFO,
                format_args!(
                    "PotatoStamps: native primitive selected key={} topology={}",
                    self.selected_mode + 1,
                    PrimitiveMode::ALL[self.selected_mode].label(),
                ),
            );
        }
        Ok(())
    }
}

fn execution_vertex_bytes(scene: &Scene) -> Vec<u8> {
    let mut vertices = Vec::with_capacity(VERTEX_COUNT * 12);
    for position in &scene.positions {
        vertices.extend_from_slice(&position.x.to_le_bytes());
        vertices.extend_from_slice(&position.y.to_le_bytes());
        vertices.extend_from_slice(&position.z.to_le_bytes());
    }
    vertices
}

fn execution_index_bytes(catalogue: &ExecutionIndexCatalogue) -> Vec<u8> {
    let mut indices = Vec::with_capacity(EXECUTION_INDEX_COUNT * core::mem::size_of::<u32>());
    for index in catalogue.indices {
        indices.extend_from_slice(&index.to_le_bytes());
    }
    indices
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
    InvalidPalette,
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
            Self::InvalidPalette => f.write_str("Picasso returned an invalid color palette"),
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
    loop {
        vsys::poll_once();
        match demo.render_frame() {
            Ok(()) => break,
            Err(error) if transient_frame_error(&error) => vsys::sleep_ms(16),
            Err(error) => return Err(error),
        }
    }
    logl::log(
        level::INFO,
        format_args!(
            "PotatoStamps: indexed primitive frame submitted and retired mode={} timeline={}",
            PrimitiveMode::ALL[demo.selected_mode].label(),
            demo.timeline,
        ),
    );
    let start = clock::monotonic_millis();
    loop {
        vsys::poll_once();
        vsys::sleep_ms(16);
        // UI4 may still own the previous lease for a scheduling instant even
        // after GPU retirement. Busy is transient; preserve the app and retry.
        match demo.render_frame() {
            Ok(()) => {}
            Err(error) if transient_frame_error(&error) => {}
            Err(error) => return Err(error),
        }
        let _elapsed = clock::monotonic_millis().saturating_sub(start);
    }
}

fn transient_frame_error(error: &DemoError) -> bool {
    matches!(error, DemoError::Ui4("frame-begin", Ui4Error::Busy))
        || matches!(error, DemoError::Vgpu(_, code) if *code == trueos::vgpu::ERR_BUSY)
}
