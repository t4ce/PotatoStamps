//! Re-exportable authored scene document for the Potato Stamps example.
//!
//! The document is stored exactly in Picasso. This module only validates the
//! bytes returned across that storage boundary and produces the GPU-ready view
//! afterwards.

use core::fmt;
use trueos::vgpu::{IndexedBatchDrawV2, IndexedDrawBatchV2};

pub const DOCUMENT_NAME: &str = "potato-stamps.pscene";
pub const DOCUMENT_BYTES: &[u8] = include_bytes!("../Assets/potato-stamps.pscene");
pub const COLOR_TEXTURE_NAME: &str = "potato-stamps-colors.bmp";
pub const LINE_GRID_NAME: &str = "potato-stamps-line-grid.xyz";
pub const STAMP_COUNT: usize = 4;
pub const VERTICES_PER_STAMP: usize = 3;
pub const VERTEX_COUNT: usize = STAMP_COUNT * VERTICES_PER_STAMP;
pub const INDEX_COUNT: usize = VERTEX_COUNT;
/// Key 9 restores the small, clip-space 4×4 native QUADSTRIP demonstration.
pub const QUAD_GRID_SIDE_QUADS: usize = 4;
pub const QUAD_GRID_SIDE_VERTICES: usize = QUAD_GRID_SIDE_QUADS + 1;
pub const QUAD_GRID_VERTEX_OFFSET: usize = VERTEX_COUNT;
pub const QUAD_GRID_VERTEX_COUNT: usize = QUAD_GRID_SIDE_VERTICES * QUAD_GRID_SIDE_VERTICES;
pub const QUAD_STRIP_DRAW_COUNT: usize = QUAD_GRID_SIDE_QUADS;
pub const QUAD_STRIP_INDICES_PER_DRAW: usize = QUAD_GRID_SIDE_VERTICES * 2;
/// A 32×32 plane of equally spaced source vertices. Every regular and `_ADJ`
/// line/triangle-grid mode selects from this exact range rather than
/// inventing topology-specific positions.
pub const LINE_GRID_COLUMNS: usize = 32;
pub const LINE_GRID_ROWS: usize = 32;
pub const LINE_GRID_VERTEX_COUNT: usize = LINE_GRID_COLUMNS * LINE_GRID_ROWS;
pub const LINE_GRID_VERTEX_OFFSET: usize = QUAD_GRID_VERTEX_OFFSET + QUAD_GRID_VERTEX_COUNT;
pub const LINE_GRID_XYZ_BYTES: usize = LINE_GRID_VERTEX_COUNT * 12;
pub const RGB_COLOR_COUNT: usize = 3;
/// Key 1 partitions the source-vertex ordinals into two immediate-color
/// draws.  Ordinals divisible by three are green; the remaining two are red.
pub const POINT_GRID_GREEN_INDEX_COUNT: usize = (LINE_GRID_VERTEX_COUNT + 2) / 3;
pub const POINT_GRID_RED_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT - POINT_GRID_GREEN_INDEX_COUNT;
pub const LINE_GRID_TRIANGLE_LIST_INDEX_COUNT: usize =
    (LINE_GRID_COLUMNS - 1) * (LINE_GRID_ROWS - 1) * 6;
pub const LINE_GRID_TRIANGLE_COUNT: usize = LINE_GRID_TRIANGLE_LIST_INDEX_COUNT / 3;
pub const TRIANGLE_LIST_COLOR0_INDEX_COUNT: usize = LINE_GRID_TRIANGLE_COUNT.div_ceil(3) * 3;
pub const TRIANGLE_LIST_COLOR1_INDEX_COUNT: usize = (LINE_GRID_TRIANGLE_COUNT + 1) / 3 * 3;
pub const TRIANGLE_LIST_COLOR2_INDEX_COUNT: usize = LINE_GRID_TRIANGLE_COUNT / 3 * 3;
/// Adjacency selection remains exposed, but its current catalogue entry is a
/// single minimum-size degenerate primitive. Until the geometry-shader path is
/// understood, selecting any `_ADJ` form deliberately produces a clear-only
/// frame instead of pretending the extra inputs are visible geometry.
pub const LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT: usize = 4;
pub const LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT: usize = 4;
pub const LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT: usize = 6;
/// One continuous snake strip covers each adjacent pair of grid rows. It
/// reuses the join vertices between row pairs, so it references the same
/// 1,024 seeded positions while emitting the entire surface.
pub const LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT: usize =
    (LINE_GRID_ROWS - 1) * LINE_GRID_COLUMNS * 2;
/// Minimum native TRISTRIP_ADJ input count for the clear-only placeholder.
pub const LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT: usize = 6;
/// One full-plane triangle-fan interpretation consumes every seed once.
pub const LINE_GRID_TRIANGLE_FAN_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT;
/// Key 6 cycles these independent fan sizes.
pub const TRIANGLE_FAN_VERTEX_COUNTS: [usize; 7] = [1024, 512, 256, 64, 32, 16, 8];
pub const TRIANGLE_FAN_CATALOGUE_INDEX_COUNT: usize =
    LINE_GRID_VERTEX_COUNT * TRIANGLE_FAN_VERTEX_COUNTS.len();
/// Key 8 reads the same seed plane as the other native-grid modes. Its
/// 31×31 cells are independent Intel QUADLIST primitives, split 481/480
/// between two opaque draws by checkerboard parity.
pub const QUAD_LIST_CELL_COUNT: usize = (LINE_GRID_COLUMNS - 1) * (LINE_GRID_ROWS - 1);
pub const QUAD_LIST_COLOR_DRAW_COUNT: usize = 2;
pub const QUAD_LIST_COLOR0_CELL_COUNT: usize = QUAD_LIST_CELL_COUNT.div_ceil(2);
pub const QUAD_LIST_COLOR1_CELL_COUNT: usize = QUAD_LIST_CELL_COUNT / 2;
pub const QUAD_LIST_COLOR0_INDEX_COUNT: usize = QUAD_LIST_COLOR0_CELL_COUNT * 4;
pub const QUAD_LIST_COLOR1_INDEX_COUNT: usize = QUAD_LIST_COLOR1_CELL_COUNT * 4;
pub const QUAD_LIST_INDEX_COUNT: usize = QUAD_LIST_CELL_COUNT * 4;
/// Key 0 is a hardware RECTLIST. It needs screen-space input, so it owns a
/// separate set of three vertices per rectangle: lower-right, lower-left,
/// upper-left. The SF unit derives upper-right.
pub const RECT_LIST_SCREEN_WIDTH_PX: f32 = 640.0;
pub const RECT_LIST_SCREEN_HEIGHT_PX: f32 = 360.0;
pub const RECT_LIST_VERTEX_OFFSET: usize = LINE_GRID_VERTEX_OFFSET + LINE_GRID_VERTEX_COUNT;
pub const RECT_LIST_VERTICES_PER_RECTANGLE: usize = 3;
pub const RECT_LIST_CELL_COUNT: usize = QUAD_LIST_CELL_COUNT;
pub const RECT_LIST_VERTEX_COUNT: usize = RECT_LIST_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const RECT_LIST_COLOR_DRAW_COUNT: usize = 2;
pub const RECT_LIST_COLOR0_CELL_COUNT: usize = RECT_LIST_CELL_COUNT.div_ceil(2);
pub const RECT_LIST_COLOR1_CELL_COUNT: usize = RECT_LIST_CELL_COUNT / 2;
pub const RECT_LIST_COLOR0_INDEX_COUNT: usize =
    RECT_LIST_COLOR0_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const RECT_LIST_COLOR1_INDEX_COUNT: usize =
    RECT_LIST_COLOR1_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const RECT_LIST_INDEX_COUNT: usize = RECT_LIST_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const EXECUTION_VERTEX_COUNT: usize = RECT_LIST_VERTEX_OFFSET + RECT_LIST_VERTEX_COUNT;
pub const PRIMITIVE_MODE_COUNT: usize = 13;
pub const EXECUTION_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT * 3
    + LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT
    + LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT
    + LINE_GRID_TRIANGLE_LIST_INDEX_COUNT
    + LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT
    + LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT
    + LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT
    + TRIANGLE_FAN_CATALOGUE_INDEX_COUNT
    + QUAD_LIST_INDEX_COUNT
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
    LineList,
    LineListAdj,
    LineStrip,
    LineStripAdj,
    TriangleList,
    TriangleListAdj,
    TriangleStrip,
    TriangleStripAdj,
    TriangleFan,
    QuadList,
    QuadStrip,
    RectList,
}

impl PrimitiveMode {
    pub const ALL: [Self; PRIMITIVE_MODE_COUNT] = [
        Self::PointList,
        Self::LineList,
        Self::LineListAdj,
        Self::LineStrip,
        Self::LineStripAdj,
        Self::TriangleList,
        Self::TriangleListAdj,
        Self::TriangleStrip,
        Self::TriangleStripAdj,
        Self::TriangleFan,
        Self::QuadList,
        Self::QuadStrip,
        Self::RectList,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PointList => "point-list-grid",
            Self::LineList => "line-list-grid",
            Self::LineListAdj => "line-list-adj-grid",
            Self::LineStrip => "line-strip-grid",
            Self::LineStripAdj => "line-strip-adj-grid",
            Self::TriangleList => "triangle-list-grid",
            Self::TriangleListAdj => "triangle-list-adj-grid",
            Self::TriangleStrip => "triangle-strip",
            Self::TriangleStripAdj => "triangle-strip-adj-grid",
            Self::TriangleFan => "triangle-fan",
            Self::QuadList => "quad-list-checkerboard-grid",
            Self::QuadStrip => "quad-strip-4x4-grid",
            Self::RectList => "rect-list-screen-checkerboard-grid",
        }
    }

    pub const fn indices_per_draw(self) -> usize {
        match self {
            // Point-list's two modulo-three partitions deliberately have
            // unequal ranges. `draw_batch` supplies their exact descriptors.
            Self::PointList => POINT_GRID_GREEN_INDEX_COUNT,
            Self::LineList | Self::LineStrip => LINE_GRID_VERTEX_COUNT,
            Self::LineListAdj => LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT,
            Self::LineStripAdj => LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT,
            Self::TriangleList => TRIANGLE_LIST_COLOR0_INDEX_COUNT,
            Self::TriangleListAdj => LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT,
            Self::TriangleStrip => LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT,
            Self::TriangleStripAdj => LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT,
            Self::TriangleFan => LINE_GRID_TRIANGLE_FAN_INDEX_COUNT,
            Self::QuadList => QUAD_LIST_COLOR0_INDEX_COUNT,
            Self::QuadStrip => QUAD_STRIP_INDICES_PER_DRAW,
            Self::RectList => RECT_LIST_COLOR0_INDEX_COUNT,
        }
    }

    pub const fn draw_count(self) -> usize {
        match self {
            Self::PointList => 2,
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
            // Four horizontal strips use the authored RGBA palette in order.
            Self::QuadStrip => QUAD_STRIP_DRAW_COUNT,
            // The screen-space rectangles use the same red/green parity.
            Self::RectList => RECT_LIST_COLOR_DRAW_COUNT,
        }
    }

    pub const fn slot(self) -> usize {
        match self {
            Self::PointList => 0,
            Self::LineList => 1,
            Self::LineListAdj => 2,
            Self::LineStrip => 3,
            Self::LineStripAdj => 4,
            Self::TriangleList => 5,
            Self::TriangleListAdj => 6,
            Self::TriangleStrip => 7,
            Self::TriangleStripAdj => 8,
            Self::TriangleFan => 9,
            Self::QuadList => 10,
            Self::QuadStrip => 11,
            Self::RectList => 12,
        }
    }

    /// Top-row number key reserved for this native topology. The four `_ADJ`
    /// forms deliberately share Keys 2-5 with their ordinary counterpart;
    /// pressing the same key while it is selected toggles the interpretation.
    pub const fn number_key(self) -> u8 {
        match self {
            Self::PointList => 1,
            Self::LineList | Self::LineListAdj => 2,
            Self::LineStrip | Self::LineStripAdj => 3,
            Self::TriangleList | Self::TriangleListAdj => 4,
            Self::TriangleStrip | Self::TriangleStripAdj => 5,
            Self::TriangleFan => 6,
            Self::QuadList => 8,
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
            Self::PointList => trueos::vgpu::PRIMITIVE_TOPOLOGY_POINT_LIST,
            Self::LineList => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST,
            Self::LineListAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST_ADJ,
            Self::LineStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_STRIP,
            Self::LineStripAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_STRIP_ADJ,
            Self::TriangleList => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            Self::TriangleListAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_ADJ,
            Self::TriangleStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            Self::TriangleStripAdj => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_ADJ,
            Self::TriangleFan => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
            Self::QuadList => trueos::vgpu::PRIMITIVE_TOPOLOGY_QUAD_LIST,
            Self::QuadStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_QUAD_STRIP,
            Self::RectList => trueos::vgpu::PRIMITIVE_TOPOLOGY_RECT_LIST,
        }
    }

    pub const fn requires_adjacency_topology_rendering(self) -> bool {
        matches!(
            self,
            Self::LineListAdj | Self::LineStripAdj | Self::TriangleListAdj | Self::TriangleStripAdj
        )
    }

    /// Select a mode for one newly pressed top-row key.  Repeated Keys 2-5
    /// toggle exactly their ordinary and native-adjacency interpretations;
    /// every other key selects its one mode directly.
    pub const fn on_number_key_pressed(self, key: u8) -> Option<Self> {
        let next = match key {
            0 => Self::RectList,
            1 => Self::PointList,
            2 => match self {
                Self::LineList => Self::LineListAdj,
                Self::LineListAdj => Self::LineList,
                _ => Self::LineList,
            },
            3 => match self {
                Self::LineStrip => Self::LineStripAdj,
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
            8 => Self::QuadList,
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
                for index in 0..mode.indices_per_draw() {
                    indices[cursor + index] = line_grid_vertex(0);
                }
                cursor += mode.indices_per_draw();
                continue;
            }
            if mode == PrimitiveMode::PointList {
                // Store each immediate-color partition contiguously.  The
                // source ordinal, rather than screen coordinate, controls the
                // repeating green, red, red pattern.
                for grid_vertex in 0..LINE_GRID_VERTEX_COUNT {
                    if grid_vertex % RGB_COLOR_COUNT == 0 {
                        indices[cursor] = line_grid_vertex(grid_vertex);
                        cursor += 1;
                    }
                }
                debug_assert_eq!(
                    cursor,
                    first_indices[mode_slot] as usize + POINT_GRID_GREEN_INDEX_COUNT
                );
                for grid_vertex in 0..LINE_GRID_VERTEX_COUNT {
                    if grid_vertex % RGB_COLOR_COUNT != 0 {
                        indices[cursor] = line_grid_vertex(grid_vertex);
                        cursor += 1;
                    }
                }
                continue;
            }
            if mode == PrimitiveMode::LineList {
                for grid_vertex in 0..LINE_GRID_VERTEX_COUNT {
                    indices[cursor + grid_vertex] = line_grid_vertex(grid_vertex);
                }
                cursor += LINE_GRID_VERTEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::LineStrip {
                // Visit each row in the opposite direction from its neighbor.
                // This is one continuous strip using every seed vertex exactly
                // once, with short vertical connections at the two edges.
                for step in 0..LINE_GRID_VERTEX_COUNT {
                    indices[cursor + step] = line_grid_vertex(line_grid_snake_seed_vertex(step));
                }
                cursor += LINE_GRID_VERTEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleList {
                // Two CCW triangles per cell. Pack their indices into three
                // contiguous draws by source-triangle ordinal, cycling
                // red -> green -> blue while preserving opaque per-draw color.
                let first_triangle_index = cursor;
                for color in 0..RGB_COLOR_COUNT {
                    let (color_offset, color_count) = match color {
                        0 => (0, TRIANGLE_LIST_COLOR0_INDEX_COUNT),
                        1 => (
                            TRIANGLE_LIST_COLOR0_INDEX_COUNT,
                            TRIANGLE_LIST_COLOR1_INDEX_COUNT,
                        ),
                        _ => (
                            TRIANGLE_LIST_COLOR0_INDEX_COUNT
                                + TRIANGLE_LIST_COLOR1_INDEX_COUNT,
                            TRIANGLE_LIST_COLOR2_INDEX_COUNT,
                        ),
                    };
                    let mut color_cursor = first_triangle_index + color_offset;
                    for row in 0..LINE_GRID_ROWS - 1 {
                        for column in 0..LINE_GRID_COLUMNS - 1 {
                            let lower_left = line_grid_vertex(row * LINE_GRID_COLUMNS + column);
                            let lower_right =
                                line_grid_vertex(row * LINE_GRID_COLUMNS + column + 1);
                            let upper_left =
                                line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column);
                            let upper_right =
                                line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column + 1);
                            let triangle = ((row * (LINE_GRID_COLUMNS - 1) + column) * 2) % 3;
                            if triangle == color {
                                indices[color_cursor..color_cursor + 3].copy_from_slice(&[
                                    lower_left,
                                    lower_right,
                                    upper_right,
                                ]);
                                color_cursor += 3;
                            }
                            if (triangle + 1) % 3 == color {
                                indices[color_cursor..color_cursor + 3].copy_from_slice(&[
                                    lower_left,
                                    upper_right,
                                    upper_left,
                                ]);
                                color_cursor += 3;
                            }
                        }
                    }
                    debug_assert_eq!(
                        color_cursor,
                        first_triangle_index + color_offset + color_count
                    );
                }
                cursor += LINE_GRID_TRIANGLE_LIST_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleStrip {
                // Each row pair is woven from the same seed positions. The
                // direction and top/bottom ordering alternate so every
                // non-degenerate strip triangle remains CCW. At a row join
                // the repeated edge positions form two degenerate triangles,
                // then the next 31×1 cell band starts without a new draw.
                for step in 0..LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT {
                    indices[cursor + step] = triangle_strip_main_vertex(step);
                }
                cursor += LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleFan {
                // Every stage spatially partitions the same 32×32 seed plane.
                // The seven tile layouts are 32×32, 16×32, 16×16, 8×8,
                // 8×4, 4×4, and 4×2: 1 through 128 independent fans.
                for (fan_index, &fan_size) in TRIANGLE_FAN_VERTEX_COUNTS.iter().enumerate() {
                    fan_first_indices[fan_index] = cursor as u32;
                    fan_index_counts[fan_index] = fan_size as u32;
                    let fan_count = LINE_GRID_VERTEX_COUNT / fan_size;
                    for fan in 0..fan_count {
                        for slot in 0..fan_size {
                            indices[cursor] = line_grid_vertex(
                                triangle_fan_partition_seed_vertex(fan_size, fan, slot),
                            );
                            cursor += 1;
                        }
                    }
                }
                continue;
            }
            if mode == PrimitiveMode::QuadList {
                // Intel QUADLIST consumes four independent vertices per
                // primitive. Preserve the ordinary CCW cell order here:
                // lower-left, lower-right, upper-right, upper-left. Packing
                // even cells first and odd cells second makes two opaque
                // constant-color draws form the checkerboard without adding
                // vertices or a topology-specific virtual surface.
                let first_quad_index = cursor;
                for color in 0..QUAD_LIST_COLOR_DRAW_COUNT {
                    let color_first = if color == 0 {
                        first_quad_index
                    } else {
                        first_quad_index + QUAD_LIST_COLOR0_INDEX_COUNT
                    };
                    let mut color_cursor = color_first;
                    for row in 0..LINE_GRID_ROWS - 1 {
                        for column in 0..LINE_GRID_COLUMNS - 1 {
                            if (row + column) % QUAD_LIST_COLOR_DRAW_COUNT != color {
                                continue;
                            }
                            let lower_left = line_grid_vertex(row * LINE_GRID_COLUMNS + column);
                            let lower_right =
                                line_grid_vertex(row * LINE_GRID_COLUMNS + column + 1);
                            let upper_right =
                                line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column + 1);
                            let upper_left =
                                line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column);
                            indices[color_cursor..color_cursor + 4].copy_from_slice(&[
                                lower_left,
                                lower_right,
                                upper_right,
                                upper_left,
                            ]);
                            color_cursor += 4;
                        }
                    }
                    debug_assert_eq!(
                        color_cursor,
                        color_first
                            + if color == 0 {
                                QUAD_LIST_COLOR0_INDEX_COUNT
                            } else {
                                QUAD_LIST_COLOR1_INDEX_COUNT
                            }
                    );
                }
                cursor += QUAD_LIST_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::QuadStrip {
                // TGL PRM: [0] = (0, 1, 3, 2), [1] = (2, 3, 5, 4).
                // Top/bottom pairs make the small virtual 4×4 layer CCW.
                for draw in 0..QUAD_STRIP_DRAW_COUNT {
                    for column in 0..QUAD_GRID_SIDE_VERTICES {
                        indices[cursor + column * 2] = quad_grid_vertex(draw + 1, column);
                        indices[cursor + column * 2 + 1] = quad_grid_vertex(draw, column);
                    }
                    cursor += QUAD_STRIP_INDICES_PER_DRAW;
                }
                continue;
            }
            if mode == PrimitiveMode::RectList {
                // RECTLIST is specifically a screen-space native primitive.
                // Each three-index rectangle is lower-right, lower-left,
                // upper-left; hardware derives upper-right. As with Key 8,
                // parity packs red then green rectangles into two draws.
                let first_rect_index = cursor;
                for color in 0..RECT_LIST_COLOR_DRAW_COUNT {
                    let color_first = if color == 0 {
                        first_rect_index
                    } else {
                        first_rect_index + RECT_LIST_COLOR0_INDEX_COUNT
                    };
                    let mut color_cursor = color_first;
                    for row in 0..LINE_GRID_ROWS - 1 {
                        for column in 0..LINE_GRID_COLUMNS - 1 {
                            if (row + column) % RECT_LIST_COLOR_DRAW_COUNT != color {
                                continue;
                            }
                            let rectangle = row * (LINE_GRID_COLUMNS - 1) + column;
                            let lower_right = rect_list_vertex(rectangle, 0);
                            let lower_left = rect_list_vertex(rectangle, 1);
                            let upper_left = rect_list_vertex(rectangle, 2);
                            indices[color_cursor..color_cursor + 3].copy_from_slice(&[
                                lower_right,
                                lower_left,
                                upper_left,
                            ]);
                            color_cursor += 3;
                        }
                    }
                    debug_assert_eq!(
                        color_cursor,
                        color_first
                            + if color == 0 {
                                RECT_LIST_COLOR0_INDEX_COUNT
                            } else {
                                RECT_LIST_COLOR1_INDEX_COUNT
                            }
                    );
                }
                cursor += RECT_LIST_INDEX_COUNT;
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

/// Fixed clip-space virtual-voxel surface used only by Key 9's QUADSTRIP.
pub fn quad_grid_positions() -> [Position; QUAD_GRID_VERTEX_COUNT] {
    core::array::from_fn(|vertex| {
        let row = vertex / QUAD_GRID_SIDE_VERTICES;
        let column = vertex % QUAD_GRID_SIDE_VERTICES;
        Position {
            x: -0.8 + column as f32 * 0.4,
            y: -0.8 + row as f32 * 0.4,
            z: 0.0,
            texture_u: 0.0,
            texture_v: 0.0,
        }
    })
}

/// Equal XY spacing, Z=0, shared by every seeded primitive-grid mode.
pub fn line_grid_positions() -> [Position; LINE_GRID_VERTEX_COUNT] {
    core::array::from_fn(|vertex| {
        let row = vertex / LINE_GRID_COLUMNS;
        let column = vertex % LINE_GRID_COLUMNS;
        Position {
            x: -0.95 + column as f32 * (1.9 / (LINE_GRID_COLUMNS - 1) as f32),
            y: -0.95 + row as f32 * (1.9 / (LINE_GRID_ROWS - 1) as f32),
            z: 0.0,
            texture_u: 0.0,
            texture_v: 0.0,
        }
    })
}

/// Native RECTLIST source positions. The SF viewport transform is disabled
/// for this topology, so these are target-pixel centres instead of NDC.
pub fn rect_list_positions() -> [Position; RECT_LIST_VERTEX_COUNT] {
    core::array::from_fn(|vertex| {
        let rectangle = vertex / RECT_LIST_VERTICES_PER_RECTANGLE;
        let corner = vertex % RECT_LIST_VERTICES_PER_RECTANGLE;
        let row = rectangle / (LINE_GRID_COLUMNS - 1);
        let column = rectangle % (LINE_GRID_COLUMNS - 1);
        let left = 0.5
            + column as f32 * ((RECT_LIST_SCREEN_WIDTH_PX - 1.0) / (LINE_GRID_COLUMNS - 1) as f32);
        let right = 0.5
            + (column + 1) as f32
                * ((RECT_LIST_SCREEN_WIDTH_PX - 1.0) / (LINE_GRID_COLUMNS - 1) as f32);
        let upper =
            0.5 + row as f32 * ((RECT_LIST_SCREEN_HEIGHT_PX - 1.0) / (LINE_GRID_ROWS - 1) as f32);
        let lower = 0.5
            + (row + 1) as f32 * ((RECT_LIST_SCREEN_HEIGHT_PX - 1.0) / (LINE_GRID_ROWS - 1) as f32);
        let (x, y) = match corner {
            0 => (right, lower), // V0: lower-right
            1 => (left, lower),  // V1: lower-left
            2 => (left, upper),  // V2: upper-left
            _ => unreachable!("rectangle-list corner must be below three"),
        };
        Position {
            x,
            y,
            z: 0.0,
            texture_u: 0.0,
            texture_v: 0.0,
        }
    })
}

const fn quad_grid_vertex(row: usize, column: usize) -> u32 {
    (QUAD_GRID_VERTEX_OFFSET + row * QUAD_GRID_SIDE_VERTICES + column) as u32
}

const fn line_grid_vertex(vertex: usize) -> u32 {
    (LINE_GRID_VERTEX_OFFSET + vertex) as u32
}

/// Snake order for the ordinary LINESTRIP across the 32×32 seed grid.
const fn line_grid_snake_seed_vertex(step: usize) -> usize {
    let row = step / LINE_GRID_COLUMNS;
    let column_in_row = step % LINE_GRID_COLUMNS;
    let column = if row.is_multiple_of(2) {
        column_in_row
    } else {
        LINE_GRID_COLUMNS - 1 - column_in_row
    };
    row * LINE_GRID_COLUMNS + column
}

/// One ordinary Key 5 strip vertex. There are two vertices per grid column
/// in each of the 24 row bands.
const fn triangle_strip_main_seed_vertex(step: usize) -> usize {
    let vertices_per_band = LINE_GRID_COLUMNS * 2;
    let row = step / vertices_per_band;
    let pair_step = step % vertices_per_band;
    let column_step = pair_step / 2;
    let column = if row.is_multiple_of(2) {
        column_step
    } else {
        LINE_GRID_COLUMNS - 1 - column_step
    };
    let first_in_pair = pair_step.is_multiple_of(2);
    let source_row = if row.is_multiple_of(2) {
        if first_in_pair { row + 1 } else { row }
    } else if first_in_pair {
        row
    } else {
        row + 1
    };
    source_row * LINE_GRID_COLUMNS + column
}

const fn triangle_strip_main_vertex(step: usize) -> u32 {
    line_grid_vertex(triangle_strip_main_seed_vertex(step))
}

const fn rect_list_vertex(rectangle: usize, corner: usize) -> u32 {
    (RECT_LIST_VERTEX_OFFSET + rectangle * RECT_LIST_VERTICES_PER_RECTANGLE + corner) as u32
}

/// Map one fan-local slot into its rectangular spatial partition. Every stage
/// consumes all 1,024 seeds exactly once.
fn triangle_fan_partition_seed_vertex(fan_size: usize, fan: usize, slot: usize) -> usize {
    let (tile_width, tile_height) = match fan_size {
        1024 => (32, 32),
        512 => (16, 32),
        256 => (16, 16),
        64 => (8, 8),
        32 => (8, 4),
        16 => (4, 4),
        8 => (4, 2),
        _ => unreachable!("unsupported triangle-fan partition size"),
    };
    let tile_columns = LINE_GRID_COLUMNS / tile_width;
    let tile_row = fan / tile_columns;
    let tile_column = fan % tile_columns;
    let row = tile_row * tile_height + slot / tile_width;
    let column = tile_column * tile_width + slot % tile_width;
    row * LINE_GRID_COLUMNS + column
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
            assert!(range.iter().all(|&index| index == line_grid_vertex(0)));
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
    fn checkerboards_keep_all_961_cells() {
        assert_eq!(QUAD_LIST_CELL_COUNT, 961);
        assert_eq!((QUAD_LIST_COLOR0_CELL_COUNT, QUAD_LIST_COLOR1_CELL_COUNT), (481, 480));
        assert_eq!((RECT_LIST_COLOR0_CELL_COUNT, RECT_LIST_COLOR1_CELL_COUNT), (481, 480));
        let catalogue = catalogue();
        let colors = [1, 2, 3, 4];
        let quads = catalogue.draw_batch(PrimitiveMode::QuadList, colors, 0);
        assert_eq!(quads.draws[0].index_count as usize, QUAD_LIST_COLOR0_INDEX_COUNT);
        assert_eq!(quads.draws[1].index_count as usize, QUAD_LIST_COLOR1_INDEX_COUNT);
        let rects = catalogue.draw_batch(PrimitiveMode::RectList, colors, 0);
        assert_eq!(rects.draws[0].index_count as usize, RECT_LIST_COLOR0_INDEX_COUNT);
        assert_eq!(rects.draws[1].index_count as usize, RECT_LIST_COLOR1_INDEX_COUNT);
    }

    #[test]
    fn key_seven_is_unbound_and_point_list_is_default_shape() {
        assert_eq!(PrimitiveMode::PointList.on_number_key_pressed(7), None);
        assert_eq!(PrimitiveMode::ALL.len(), 13);
        let batch = catalogue().draw_batch(PrimitiveMode::PointList, [1, 2, 3, 4], 0);
        assert_eq!(batch.draw_count, 2);
        assert_eq!(batch.draws[0].index_count as usize, POINT_GRID_GREEN_INDEX_COUNT);
        assert_eq!(batch.draws[1].index_count as usize, POINT_GRID_RED_INDEX_COUNT);
    }
}
