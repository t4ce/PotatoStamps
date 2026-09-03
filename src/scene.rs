//! Re-exportable authored scene document for the Potato Stamps example.
//!
//! The document is stored exactly in Picasso. This module only validates the
//! bytes returned across that storage boundary and produces the GPU-ready view
//! afterwards.

use core::fmt;
use trueos::vgpu::{IndexedBatchDrawV2, IndexedDrawBatchV2};

mod adjacency;
mod base;

pub use base::*;

pub const DOCUMENT_NAME: &str = "potato-stamps.pscene";
pub const DOCUMENT_BYTES: &[u8] = include_bytes!("../Assets/potato-stamps.pscene");
pub const COLOR_TEXTURE_NAME: &str = "potato-stamps-colors.bmp";
pub const LINE_GRID_NAME: &str = "potato-stamps-line-grid.xyz";
pub const STAMP_COUNT: usize = 4;
pub const VERTICES_PER_STAMP: usize = 3;
pub const VERTEX_COUNT: usize = STAMP_COUNT * VERTICES_PER_STAMP;
pub const INDEX_COUNT: usize = VERTEX_COUNT;
/// The four adjacency modes currently own only minimum-size clear-only placeholders.
pub const LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT: usize = adjacency::LINE_LIST_INDEX_COUNT;
pub const LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT: usize = adjacency::LINE_STRIP_INDEX_COUNT;
pub const LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT: usize = adjacency::TRIANGLE_LIST_INDEX_COUNT;
pub const LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT: usize = adjacency::TRIANGLE_STRIP_INDEX_COUNT;
pub const PRIMITIVE_MODE_COUNT: usize = 17;
pub const EXECUTION_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT * 3
    + POINT_RING_INDEX_COUNT
    + LINE_LIST_RING_INDEX_COUNT
    + LINE_STRIP_RING_INDEX_COUNT
    + LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT
    + LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT
    + LINE_GRID_TRIANGLE_LIST_INDEX_COUNT
    + LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT
    + LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT
    + LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT
    + TRIANGLE_FAN_CATALOGUE_INDEX_COUNT
    + QUAD_LIST_INDEX_COUNT
    + QUAD_LIST_RING_INDEX_COUNT
    + QUAD_STRIP_DRAW_COUNT * QUAD_STRIP_INDICES_PER_DRAW
    + RECT_LIST_INDEX_COUNT;

/// A four-texel opaque BMP: red, green, blue, and yellow. The bytes seed
/// Picasso, then must be read back from Picasso before palette decoding.
pub const COLOR_TEXTURE_BYTES: &[u8] = &[
    b'B', b'M', 70, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, // BMP file header
    40, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 1, 0, 32, 0, // DIB core
    0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // BI_RGB + resolution
    0, 0, 0, 0, 0, 0, 0, 0, // palette counts
    0, 0, 255, 255, // red, BGRA
    0, 255, 0, 255, // green, BGRA
    255, 0, 0, 255, // blue, BGRA
    0, 255, 255, 255, // yellow, BGRA
];

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// Legacy authored palette selector retained in the re-exportable scene.
    /// The direct execution vertex buffer intentionally uploads only XYZ.
    pub texture_u: f32,
    pub texture_v: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scene {
    pub positions: [Position; VERTEX_COUNT],
    pub indices: [u32; INDEX_COUNT],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveMode {
    PointList,
    PointListRings,
    LineList,
    LineListRings,
    LineListAdj,
    LineStrip,
    LineStripRings,
    LineStripAdj,
    TriangleList,
    TriangleListAdj,
    TriangleStrip,
    TriangleStripAdj,
    TriangleFan,
    QuadList,
    QuadListRings,
    QuadStrip,
    RectList,
}

impl PrimitiveMode {
    pub const ALL: [Self; PRIMITIVE_MODE_COUNT] = [
        Self::PointList,
        Self::PointListRings,
        Self::LineList,
        Self::LineListRings,
        Self::LineListAdj,
        Self::LineStrip,
        Self::LineStripRings,
        Self::LineStripAdj,
        Self::TriangleList,
        Self::TriangleListAdj,
        Self::TriangleStrip,
        Self::TriangleStripAdj,
        Self::TriangleFan,
        Self::QuadList,
        Self::QuadListRings,
        Self::QuadStrip,
        Self::RectList,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PointList => "point-list-grid",
            Self::PointListRings => "point-list-four-circles",
            Self::LineList => "line-list-grid",
            Self::LineListRings => "line-list-four-dashed-circles",
            Self::LineListAdj => "line-list-adj-grid",
            Self::LineStrip => "line-strip-grid",
            Self::LineStripRings => "line-strip-four-circles",
            Self::LineStripAdj => "line-strip-adj-grid",
            Self::TriangleList => "triangle-list-grid",
            Self::TriangleListAdj => "triangle-list-adj-grid",
            Self::TriangleStrip => "triangle-strip",
            Self::TriangleStripAdj => "triangle-strip-adj-grid",
            Self::TriangleFan => "triangle-fan",
            Self::QuadList => "quad-list-checkerboard-grid",
            Self::QuadListRings => "quad-list-gapped-rings",
            Self::QuadStrip => "quad-strip-two-rings",
            Self::RectList => "rect-list-two-rings",
        }
    }

    pub const fn indices_per_draw(self) -> usize {
        match self {
            // Point-list's two modulo-three partitions deliberately have
            // unequal ranges. `draw_batch` supplies their exact descriptors.
            Self::PointList => POINT_GRID_GREEN_INDEX_COUNT,
            Self::PointListRings => POINT_RING_INDICES_PER_DRAW,
            Self::LineList | Self::LineStrip => LINE_GRID_VERTEX_COUNT,
            Self::LineListRings => LINE_LIST_RING_INDICES_PER_DRAW,
            Self::LineListAdj => LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT,
            Self::LineStripRings => LINE_STRIP_RING_INDICES_PER_DRAW,
            Self::LineStripAdj => LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT,
            Self::TriangleList => TRIANGLE_LIST_COLOR0_INDEX_COUNT,
            Self::TriangleListAdj => LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT,
            Self::TriangleStrip => LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT,
            Self::TriangleStripAdj => LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT,
            Self::TriangleFan => LINE_GRID_TRIANGLE_FAN_INDEX_COUNT,
            Self::QuadList => QUAD_LIST_COLOR0_INDEX_COUNT,
            Self::QuadListRings => QUAD_LIST_RING_INDICES_PER_DRAW,
            Self::QuadStrip => QUAD_STRIP_INDICES_PER_DRAW,
            Self::RectList => RECT_LIST_COLOR0_INDEX_COUNT,
        }
    }

    pub const fn draw_count(self) -> usize {
        match self {
            Self::PointList => 2,
            Self::PointListRings => POINT_RING_DRAW_COUNT,
            Self::LineListRings => LINE_LIST_RING_DRAW_COUNT,
            Self::LineStripRings => LINE_STRIP_RING_DRAW_COUNT,
            Self::LineList
            | Self::LineListAdj
            | Self::LineStrip
            | Self::LineStripAdj
            | Self::TriangleStrip
            | Self::TriangleStripAdj
            | Self::TriangleFan => 1,
            // Three opaque constant-color draws: red, green, then blue.
            Self::TriangleList => RGB_COLOR_COUNT,
            Self::TriangleListAdj => 1,
            // Red and green independent quads interleave as a checkerboard.
            Self::QuadList => QUAD_LIST_COLOR_DRAW_COUNT,
            Self::QuadListRings => QUAD_LIST_RING_DRAW_COUNT,
            // Two independent rings use the authored RGBA palette in order.
            Self::QuadStrip => QUAD_STRIP_DRAW_COUNT,
            // The screen-space rectangles use the same red/green parity.
            Self::RectList => RECT_LIST_COLOR_DRAW_COUNT,
        }
    }

    pub const fn slot(self) -> usize {
        match self {
            Self::PointList => 0,
            Self::PointListRings => 1,
            Self::LineList => 2,
            Self::LineListRings => 3,
            Self::LineListAdj => 4,
            Self::LineStrip => 5,
            Self::LineStripRings => 6,
            Self::LineStripAdj => 7,
            Self::TriangleList => 8,
            Self::TriangleListAdj => 9,
            Self::TriangleStrip => 10,
            Self::TriangleStripAdj => 11,
            Self::TriangleFan => 12,
            Self::QuadList => 13,
            Self::QuadListRings => 14,
            Self::QuadStrip => 15,
            Self::RectList => 16,
        }
    }

    /// Top-row number key reserved for this native topology. The four `_ADJ`
    /// forms deliberately share Keys 2-5 with their ordinary counterpart;
    /// pressing the same key while it is selected toggles the interpretation.
    pub const fn number_key(self) -> u8 {
        match self {
            Self::PointList | Self::PointListRings => 1,
            Self::LineList | Self::LineListRings | Self::LineListAdj => 2,
            Self::LineStrip | Self::LineStripRings | Self::LineStripAdj => 3,
            Self::TriangleList | Self::TriangleListAdj => 4,
            Self::TriangleStrip | Self::TriangleStripAdj => 5,
            Self::TriangleFan => 6,
            Self::QuadList | Self::QuadListRings => 8,
            Self::QuadStrip => 9,
            Self::RectList => 0,
        }
    }

    pub const fn number_key_hid_usage(self) -> u8 {
        // USB HID uses 0x1e..=0x26 for top-row `1`..=`9`, then 0x27 for `0`.
        match self.number_key() {
            0 => 0x27,
            key => 0x1d + key,
        }
    }

    pub const fn vgpu_topology(self) -> u32 {
        match self {
            Self::PointList | Self::PointListRings => trueos::vgpu::PRIMITIVE_TOPOLOGY_POINT_LIST,
            Self::LineList | Self::LineListRings => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST,
            Self::LineListAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST_ADJ,
            Self::LineStrip | Self::LineStripRings => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_STRIP,
            Self::LineStripAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_STRIP_ADJ,
            Self::TriangleList => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            Self::TriangleListAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_ADJ,
            Self::TriangleStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            Self::TriangleStripAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_ADJ,
            Self::TriangleFan => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
            Self::QuadList | Self::QuadListRings => trueos::vgpu::PRIMITIVE_TOPOLOGY_QUAD_LIST,
            Self::QuadStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_QUAD_STRIP,
            Self::RectList => trueos::vgpu::PRIMITIVE_TOPOLOGY_RECT_LIST,
        }
    }

    pub const fn requires_adjacency_topology_rendering(self) -> bool {
        adjacency::is_adjacency_mode(self)
    }

    /// Keys 1-3 add circle interpretations before the existing adjacency
    /// stages. Other topology toggles retain their previous cycles.
    pub const fn on_number_key_pressed(self, key: u8) -> Option<Self> {
        let next = match key {
            0 => Self::RectList,
            1 => match self {
                Self::PointList => Self::PointListRings,
                Self::PointListRings => Self::PointList,
                _ => Self::PointList,
            },
            2 => match self {
                Self::LineList => Self::LineListRings,
                Self::LineListRings => Self::LineListAdj,
                Self::LineListAdj => Self::LineList,
                _ => Self::LineList,
            },
            3 => match self {
                Self::LineStrip => Self::LineStripRings,
                Self::LineStripRings => Self::LineStripAdj,
                Self::LineStripAdj => Self::LineStrip,
                _ => Self::LineStrip,
            },
            4 => match self {
                Self::TriangleList => Self::TriangleListAdj,
                Self::TriangleListAdj => Self::TriangleList,
                _ => Self::TriangleList,
            },
            5 => match self {
                Self::TriangleStrip => Self::TriangleStripAdj,
                Self::TriangleStripAdj => Self::TriangleStrip,
                _ => Self::TriangleStrip,
            },
            6 => Self::TriangleFan,
            // Key 7 has no function.
            7 => return None,
            8 => match self {
                Self::QuadList => Self::QuadListRings,
                Self::QuadListRings => Self::QuadList,
                _ => Self::QuadList,
            },
            9 => Self::QuadStrip,
            _ => return None,
        };
        Some(next)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionIndexCatalogue {
    pub indices: [u32; EXECUTION_INDEX_COUNT],
    pub first_indices: [u32; PRIMITIVE_MODE_COUNT],
    pub fan_first_indices: [u32; TRIANGLE_FAN_VERTEX_COUNTS.len()],
    pub fan_index_counts: [u32; TRIANGLE_FAN_VERTEX_COUNTS.len()],
}

impl ExecutionIndexCatalogue {
    /// Lower one catalogue interpretation to the exact V2 descriptors consumed
    /// by the kernel. The uploaded vertex and index buffers remain unchanged.
    pub fn draw_batch(
        &self,
        mode: PrimitiveMode,
        colors: [u32; STAMP_COUNT],
        clear_rgba8_srgb: u32,
    ) -> IndexedDrawBatchV2 {
        self.draw_batch_with_fan_size(mode, colors, clear_rgba8_srgb, LINE_GRID_VERTEX_COUNT)
    }

    pub fn draw_batch_with_fan_size(
        &self,
        mode: PrimitiveMode,
        colors: [u32; STAMP_COUNT],
        clear_rgba8_srgb: u32,
        fan_vertices: usize,
    ) -> IndexedDrawBatchV2 {
        let mode_slot = mode.slot();
        let draw_count = if mode == PrimitiveMode::TriangleFan {
            LINE_GRID_VERTEX_COUNT / fan_vertices
        } else {
            mode.draw_count()
        };
        let mut batch = IndexedDrawBatchV2 {
            clear_rgba8_srgb,
            draw_count: draw_count as u32,
            ..IndexedDrawBatchV2::default()
        };
        for draw in 0..draw_count {
            let (index_count, first_index, palette_index) = if mode == PrimitiveMode::PointList {
                // Key 1 prepares the viewer for reinterpreted geometry: seed
                // ordinals 0, 3, 6, ... are green and all others are red.
                // These two disjoint index ranges use the existing immediate
                // per-draw RGBA field; no vertex format or shader ABI changes.
                if draw == 0 {
                    (
                        POINT_GRID_GREEN_INDEX_COUNT as u32,
                        self.first_indices[mode_slot],
                        1,
                    )
                } else {
                    (
                        POINT_GRID_RED_INDEX_COUNT as u32,
                        self.first_indices[mode_slot] + POINT_GRID_GREEN_INDEX_COUNT as u32,
                        0,
                    )
                }
            } else if mode == PrimitiveMode::TriangleList {
                let (index_count, first_offset) = match draw {
                    0 => (TRIANGLE_LIST_COLOR0_INDEX_COUNT, 0),
                    1 => (TRIANGLE_LIST_COLOR1_INDEX_COUNT, TRIANGLE_LIST_COLOR0_INDEX_COUNT),
                    _ => (
                        TRIANGLE_LIST_COLOR2_INDEX_COUNT,
                        TRIANGLE_LIST_COLOR0_INDEX_COUNT + TRIANGLE_LIST_COLOR1_INDEX_COUNT,
                    ),
                };
                (
                    index_count as u32,
                    self.first_indices[mode_slot] + first_offset as u32,
                    draw,
                )
            } else if mode == PrimitiveMode::QuadList {
                let (index_count, first_offset) = if draw == 0 {
                    (QUAD_LIST_COLOR0_INDEX_COUNT, 0)
                } else {
                    (QUAD_LIST_COLOR1_INDEX_COUNT, QUAD_LIST_COLOR0_INDEX_COUNT)
                };
                (
                    index_count as u32,
                    self.first_indices[mode_slot] + first_offset as u32,
                    draw,
                )
            } else if mode == PrimitiveMode::RectList {
                let (index_count, first_offset) = if draw == 0 {
                    (RECT_LIST_COLOR0_INDEX_COUNT, 0)
                } else {
                    (RECT_LIST_COLOR1_INDEX_COUNT, RECT_LIST_COLOR0_INDEX_COUNT)
                };
                (
                    index_count as u32,
                    self.first_indices[mode_slot] + first_offset as u32,
                    draw,
                )
            } else {
                (
                    if mode == PrimitiveMode::TriangleFan {
                        fan_vertices as u32
                    } else {
                        mode.indices_per_draw() as u32
                    },
                    if mode == PrimitiveMode::TriangleFan {
                        self.fan_first_indices[TRIANGLE_FAN_VERTEX_COUNTS
                            .iter()
                            .position(|&size| size == fan_vertices)
                            .expect("invalid triangle fan size")]
                    } else {
                        self.first_indices[mode_slot]
                    } + draw as u32
                        * if mode == PrimitiveMode::TriangleFan {
                            fan_vertices as u32
                        } else {
                            mode.indices_per_draw() as u32
                        },
                    if mode == PrimitiveMode::TriangleFan {
                        draw % RGB_COLOR_COUNT
                    } else {
                        draw
                    },
                )
            };
            batch.draws[draw] = IndexedBatchDrawV2 {
                index_count,
                first_index,
                base_vertex: 0,
                // Independent fans cycle the opaque RGB palette; the other
                // modes retain their authored per-draw palette selection.
                rgba8_srgb: colors[palette_index],
                topology: mode.vgpu_topology(),
                reserved: 0,
            };
        }
        batch
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneError {
    Header,
    VertexCount,
    Position,
    Indices,
    TrailingData,
}

impl fmt::Display for SceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Header => "invalid Potato Stamps scene header",
            Self::VertexCount => "invalid Potato Stamps vertex count",
            Self::Position => "invalid Potato Stamps position or texture selector",
            Self::Indices => "invalid Potato Stamps triangle indices",
            Self::TrailingData => "unexpected Potato Stamps scene data",
        })
    }
}

impl Scene {
    pub fn decode(bytes: &[u8]) -> Result<Self, SceneError> {
        let mut tokens = Tokens::new(bytes);
        if tokens.next() != Some(b"POTATO-STAMPS/2") {
            return Err(SceneError::Header);
        }
        if parse_decimal(tokens.next()).ok_or(SceneError::VertexCount)? != VERTEX_COUNT as u32 {
            return Err(SceneError::VertexCount);
        }

        let mut positions = [Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            texture_u: 0.0,
            texture_v: 0.0,
        }; VERTEX_COUNT];
        for position in &mut positions {
            *position = Position {
                x: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
                y: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
                z: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
                texture_u: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
                texture_v: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
            };
            if !position.x.is_finite()
                || !position.y.is_finite()
                || !position.z.is_finite()
                || !position.texture_u.is_finite()
                || !position.texture_v.is_finite()
            {
                return Err(SceneError::Position);
            }
        }
        if tokens.next() != Some(b"TRIANGLES") {
            return Err(SceneError::Indices);
        }
        let mut indices = [0; INDEX_COUNT];
        for index in &mut indices {
            *index = parse_decimal(tokens.next()).ok_or(SceneError::Indices)?;
            if *index as usize >= VERTEX_COUNT {
                return Err(SceneError::Indices);
            }
        }
        if tokens.next().is_some() {
            return Err(SceneError::TrailingData);
        }
        if !all_triangles_are_ccw(&positions, &indices) {
            return Err(SceneError::Indices);
        }
        Ok(Self { positions, indices })
    }

    /// Build every native primitive interpretation once from the 1,024-vertex
    /// Picasso-seeded plane. Frame submission only selects ranges from this
    /// catalogue; it never rewrites execution-buffer bytes.
    pub fn execution_index_catalogue(&self) -> ExecutionIndexCatalogue {
        let mut indices = [0; EXECUTION_INDEX_COUNT];
        let mut first_indices = [0; PRIMITIVE_MODE_COUNT];
        let mut fan_first_indices = [0u32; TRIANGLE_FAN_VERTEX_COUNTS.len()];
        let mut fan_index_counts = [0u32; TRIANGLE_FAN_VERTEX_COUNTS.len()];
        let mut cursor = 0usize;
        for (mode_slot, mode) in PrimitiveMode::ALL.into_iter().enumerate() {
            first_indices[mode_slot] = cursor as u32;
            if mode.requires_adjacency_topology_rendering() {
                adjacency::append_degenerate_indices(&mut indices, &mut cursor, mode);
                continue;
            }
            if base::append_indices(
                mode,
                &mut indices,
                &mut cursor,
                &mut fan_first_indices,
                &mut fan_index_counts,
            ) {
                continue;
            }
            unreachable!("every selectable primitive mode must have an execution mapping");
        }
        debug_assert_eq!(cursor, EXECUTION_INDEX_COUNT);
        ExecutionIndexCatalogue {
            indices,
            first_indices,
            fan_first_indices,
            fan_index_counts,
        }
    }
}

pub fn decode_palette_rgba(bytes: &[u8]) -> Option<[u32; STAMP_COUNT]> {
    if bytes.len() != COLOR_TEXTURE_BYTES.len()
        || bytes.get(..2) != Some(b"BM")
        || bytes.get(18..26) != Some(&[4, 0, 0, 0, 1, 0, 0, 0])
    {
        return None;
    }
    let pixels = bytes.get(54..70)?;
    Some(core::array::from_fn(|slot| {
        let offset = slot * 4;
        u32::from_le_bytes([
            pixels[offset + 2],
            pixels[offset + 1],
            pixels[offset],
            pixels[offset + 3],
        ])
    }))
}

fn all_triangles_are_ccw(
    positions: &[Position; VERTEX_COUNT],
    indices: &[u32; INDEX_COUNT],
) -> bool {
    indices.chunks_exact(VERTICES_PER_STAMP).all(|triangle| {
        let a = positions[triangle[0] as usize];
        let b = positions[triangle[1] as usize];
        let c = positions[triangle[2] as usize];
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) > 0.0
    })
}

fn parse_decimal(token: Option<&[u8]>) -> Option<u32> {
    let token = token?;
    (!token.is_empty()).then_some(())?;
    token.iter().try_fold(0u32, |value, digit| {
        digit
            .checked_sub(b'0')
            .filter(|digit| *digit <= 9)
            .and_then(|digit| value.checked_mul(10)?.checked_add(u32::from(digit)))
    })
}

fn parse_hex_f32(token: Option<&[u8]>) -> Option<f32> {
    let token = token?;
    if token.len() != 8 {
        return None;
    }
    let bits = token.iter().try_fold(0u32, |value, digit| {
        let nibble = match digit {
            b'0'..=b'9' => digit - b'0',
            b'A'..=b'F' => digit - b'A' + 10,
            b'a'..=b'f' => digit - b'a' + 10,
            _ => return None,
        };
        value.checked_mul(16)?.checked_add(u32::from(nibble))
    })?;
    Some(f32::from_bits(bits))
}

struct Tokens<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Tokens<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn next(&mut self) -> Option<&'a [u8]> {
        while self.cursor < self.bytes.len() && self.bytes[self.cursor].is_ascii_whitespace() {
            self.cursor += 1;
        }
        let start = self.cursor;
        while self.cursor < self.bytes.len() && !self.bytes[self.cursor].is_ascii_whitespace() {
            self.cursor += 1;
        }
        (start != self.cursor).then(|| &self.bytes[start..self.cursor])
    }
}

#[cfg(test)]
mod current_tests {
    use super::*;

    fn catalogue() -> ExecutionIndexCatalogue {
        Scene::decode(DOCUMENT_BYTES)
            .unwrap()
            .execution_index_catalogue()
    }

    #[test]
    fn seed_plane_is_32_by_32() {
        assert_eq!(LINE_GRID_COLUMNS, 32);
        assert_eq!(LINE_GRID_ROWS, 32);
        assert_eq!(LINE_GRID_VERTEX_COUNT, 1024);
        assert_eq!(line_grid_positions().len(), 1024);
        assert_eq!(LINE_GRID_XYZ_BYTES, 1024 * 12);
    }

    #[test]
    fn adjacency_modes_are_minimum_size_degenerate_draws() {
        let catalogue = catalogue();
        for mode in [
            PrimitiveMode::LineListAdj,
            PrimitiveMode::LineStripAdj,
            PrimitiveMode::TriangleListAdj,
            PrimitiveMode::TriangleStripAdj,
        ] {
            let first = catalogue.first_indices[mode.slot()] as usize;
            let count = mode.indices_per_draw();
            let range = &catalogue.indices[first..first + count];
            assert!(range.iter().all(|&index| index == base::line_grid_vertex(0)));
            let batch = catalogue.draw_batch(mode, [1, 2, 3, 4], 0);
            assert_eq!(batch.draw_count, 1);
            assert_eq!(batch.draws[0].index_count as usize, count);
            assert_eq!(batch.draws[0].topology, mode.vgpu_topology());
        }
    }

    #[test]
    fn fan_cycle_is_seven_exact_spatial_partitions() {
        assert_eq!(TRIANGLE_FAN_VERTEX_COUNTS, [1024, 512, 256, 64, 32, 16, 8]);
        let catalogue = catalogue();
        for (stage, &fan_size) in TRIANGLE_FAN_VERTEX_COUNTS.iter().enumerate() {
            let first = catalogue.fan_first_indices[stage] as usize;
            let stage_indices = &catalogue.indices[first..first + LINE_GRID_VERTEX_COUNT];
            let mut seen = [false; LINE_GRID_VERTEX_COUNT];
            for &index in stage_indices {
                let seed = index as usize - LINE_GRID_VERTEX_OFFSET;
                assert!(!seen[seed]);
                seen[seed] = true;
            }
            assert!(seen.into_iter().all(|value| value));

            let batch = catalogue.draw_batch_with_fan_size(
                PrimitiveMode::TriangleFan,
                [1, 2, 3, 4],
                0,
                fan_size,
            );
            assert_eq!(batch.draw_count as usize, LINE_GRID_VERTEX_COUNT / fan_size);
            assert_eq!(batch.draws[0].index_count as usize, fan_size);
        }
    }

    #[test]
    fn quad_list_consumes_every_seed_once_and_rect_list_has_two_ring_draws() {
        assert_eq!(QUAD_LIST_CELL_COUNT, 256);
        assert_eq!((QUAD_LIST_COLOR0_CELL_COUNT, QUAD_LIST_COLOR1_CELL_COUNT), (128, 128));
        assert_eq!(QUAD_LIST_INDEX_COUNT, LINE_GRID_VERTEX_COUNT);
        assert_eq!(RECT_LIST_CELL_COUNT, 128);
        assert_eq!((RECT_LIST_COLOR0_CELL_COUNT, RECT_LIST_COLOR1_CELL_COUNT), (64, 64));
        let catalogue = catalogue();
        let first = catalogue.first_indices[PrimitiveMode::QuadList.slot()] as usize;
        let indices = &catalogue.indices[first..first + QUAD_LIST_INDEX_COUNT];
        let mut seen = [false; LINE_GRID_VERTEX_COUNT];
        for &index in indices {
            let seed = index as usize - LINE_GRID_VERTEX_OFFSET;
            assert!(!seen[seed]);
            seen[seed] = true;
        }
        assert!(seen.into_iter().all(|value| value));
        let colors = [1, 2, 3, 4];
        let quads = catalogue.draw_batch(PrimitiveMode::QuadList, colors, 0);
        assert_eq!(quads.draws[0].index_count as usize, QUAD_LIST_COLOR0_INDEX_COUNT);
        assert_eq!(quads.draws[1].index_count as usize, QUAD_LIST_COLOR1_INDEX_COUNT);
        let rects = catalogue.draw_batch(PrimitiveMode::RectList, colors, 0);
        assert_eq!(rects.draws[0].index_count as usize, RECT_LIST_COLOR0_INDEX_COUNT);
        assert_eq!(rects.draws[1].index_count as usize, RECT_LIST_COLOR1_INDEX_COUNT);
    }

    #[test]
    fn rect_list_stores_three_equal_rectangle_corners_and_no_implied_fourth() {
        let positions = rect_list_positions();
        assert_eq!(positions.len(), RECT_LIST_CELL_COUNT * 3);
        for rectangle in 0..RECT_LIST_CELL_COUNT {
            let first = rectangle * RECT_LIST_VERTICES_PER_RECTANGLE;
            let v0 = positions[first];
            let v1 = positions[first + 1];
            let v2 = positions[first + 2];
            assert!((v0.x - v1.x - RECT_LIST_RECT_WIDTH_PX).abs() < 0.0001);
            assert!((v1.y - v2.y - RECT_LIST_RECT_HEIGHT_PX).abs() < 0.0001);
            assert!((v0.y - v1.y).abs() < 0.0001);
            assert!((v1.x - v2.x).abs() < 0.0001);
        }
    }

    #[test]
    fn key_seven_is_unbound_and_point_list_is_default_shape() {
        assert_eq!(PrimitiveMode::PointList.on_number_key_pressed(7), None);
        assert_eq!(PrimitiveMode::ALL.len(), 17);
        assert_eq!(
            PrimitiveMode::PointList.on_number_key_pressed(1),
            Some(PrimitiveMode::PointListRings)
        );
        assert_eq!(
            PrimitiveMode::PointListRings.on_number_key_pressed(1),
            Some(PrimitiveMode::PointList)
        );
        assert_eq!(
            PrimitiveMode::LineList.on_number_key_pressed(2),
            Some(PrimitiveMode::LineListRings)
        );
        assert_eq!(
            PrimitiveMode::LineListRings.on_number_key_pressed(2),
            Some(PrimitiveMode::LineListAdj)
        );
        assert_eq!(
            PrimitiveMode::LineStrip.on_number_key_pressed(3),
            Some(PrimitiveMode::LineStripRings)
        );
        assert_eq!(
            PrimitiveMode::LineStripRings.on_number_key_pressed(3),
            Some(PrimitiveMode::LineStripAdj)
        );
        assert_eq!(
            PrimitiveMode::QuadList.on_number_key_pressed(8),
            Some(PrimitiveMode::QuadListRings)
        );
        assert_eq!(
            PrimitiveMode::QuadListRings.on_number_key_pressed(8),
            Some(PrimitiveMode::QuadList)
        );
        let batch = catalogue().draw_batch(PrimitiveMode::PointList, [1, 2, 3, 4], 0);
        assert_eq!(batch.draw_count, 2);
        assert_eq!(batch.draws[0].index_count as usize, POINT_GRID_GREEN_INDEX_COUNT);
        assert_eq!(batch.draws[1].index_count as usize, POINT_GRID_RED_INDEX_COUNT);
    }

    #[test]
    fn point_line_and_strip_circle_modes_share_the_four_ring_paths() {
        let catalogue = catalogue();
        for mode in [
            PrimitiveMode::PointListRings,
            PrimitiveMode::LineListRings,
        ] {
            let first = catalogue.first_indices[mode.slot()] as usize;
            let indices = &catalogue.indices[first..first + QUAD_STRIP_RING_VERTEX_COUNT];
            for circle in 0..RING_CIRCLE_COUNT {
                for step in 0..RING_CIRCLE_VERTEX_COUNT {
                    assert_eq!(
                        indices[circle * RING_CIRCLE_VERTEX_COUNT + step],
                        base::ring_circle_vertex(circle, step)
                    );
                }
            }
            let batch = catalogue.draw_batch(mode, [1, 2, 3, 4], 0);
            assert_eq!(batch.draw_count as usize, RING_CIRCLE_COUNT);
        }

        let mode = PrimitiveMode::LineStripRings;
        let first = catalogue.first_indices[mode.slot()] as usize;
        for circle in 0..RING_CIRCLE_COUNT {
            let circle_first = first + circle * LINE_STRIP_RING_INDICES_PER_DRAW;
            let strip = &catalogue.indices
                [circle_first..circle_first + LINE_STRIP_RING_INDICES_PER_DRAW];
            assert_eq!(strip[0], strip[RING_CIRCLE_VERTEX_COUNT]);
            for step in 0..RING_CIRCLE_VERTEX_COUNT {
                assert_eq!(strip[step], base::ring_circle_vertex(circle, step));
            }
        }
        let batch = catalogue.draw_batch(mode, [1, 2, 3, 4], 0);
        assert_eq!(batch.draw_count as usize, RING_CIRCLE_COUNT);
        assert_eq!(
            batch.draws[0].index_count as usize,
            LINE_STRIP_RING_INDICES_PER_DRAW
        );
    }

    #[test]
    fn quad_list_rings_reuse_every_quad_strip_vertex_once() {
        let catalogue = catalogue();
        let first = catalogue.first_indices[PrimitiveMode::QuadListRings.slot()] as usize;
        let rings = &catalogue.indices[first..first + QUAD_LIST_RING_INDEX_COUNT];
        assert_eq!(QUAD_LIST_RING_CELL_COUNT, 64);
        assert_eq!(QUAD_LIST_RING_INDEX_COUNT, QUAD_STRIP_RING_VERTEX_COUNT);
        let mut seen = [false; QUAD_STRIP_RING_VERTEX_COUNT];
        for &index in rings {
            let vertex = index as usize - QUAD_STRIP_RING_VERTEX_OFFSET;
            assert!(!seen[vertex]);
            seen[vertex] = true;
        }
        assert!(seen.into_iter().all(|value| value));
        let batch = catalogue.draw_batch(PrimitiveMode::QuadListRings, [1, 2, 3, 4], 0);
        assert_eq!(batch.draw_count, 2);
        assert_eq!(batch.draws[0].index_count as usize, QUAD_LIST_RING_INDICES_PER_DRAW);
        assert_eq!(batch.draws[1].index_count as usize, QUAD_LIST_RING_INDICES_PER_DRAW);
    }

    #[test]
    fn quad_strip_is_two_independent_closed_rings() {
        let catalogue = catalogue();
        let first = catalogue.first_indices[PrimitiveMode::QuadStrip.slot()] as usize;
        assert_eq!(QUAD_STRIP_DRAW_COUNT, 2);
        assert_eq!(QUAD_STRIP_RING_VERTEX_COUNT, 256);
        let mut seen = [false; QUAD_STRIP_RING_VERTEX_COUNT];
        for ring in 0..QUAD_STRIP_RING_COUNT {
            let ring_first = first + ring * QUAD_STRIP_INDICES_PER_DRAW;
            let strip =
                &catalogue.indices[ring_first..ring_first + QUAD_STRIP_INDICES_PER_DRAW];
            assert_eq!(&strip[..2], &strip[strip.len() - 2..]);
            for &index in &strip[..QUAD_STRIP_RING_VERTICES_PER_RING] {
                let vertex = index as usize - QUAD_STRIP_RING_VERTEX_OFFSET;
                assert!(!seen[vertex]);
                seen[vertex] = true;
            }
        }
        assert!(seen.into_iter().all(|value| value));
        let batch = catalogue.draw_batch(PrimitiveMode::QuadStrip, [1, 2, 3, 4], 0);
        assert_eq!(batch.draw_count, 2);
        assert_eq!(batch.draws[0].index_count as usize, QUAD_STRIP_INDICES_PER_DRAW);
        assert_eq!(batch.draws[1].index_count as usize, QUAD_STRIP_INDICES_PER_DRAW);
    }
}
