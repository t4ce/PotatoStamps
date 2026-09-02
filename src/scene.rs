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
/// One thousand equally spaced source vertices. Every regular and `_ADJ`
/// line/triangle-grid mode selects from this exact range rather than
/// inventing topology-specific positions.
pub const LINE_GRID_COLUMNS: usize = 40;
pub const LINE_GRID_ROWS: usize = 25;
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
pub const LINE_GRID_TRIANGLE_LIST_INDICES_PER_RGB_COLOR: usize =
    LINE_GRID_TRIANGLE_LIST_INDEX_COUNT / RGB_COLOR_COUNT;
/// Key 2 changes only the topology descriptor. Both interpretations consume
/// the identical 1,000-index seed range: ordinary LINELIST assembles 500
/// lines, while LINELIST_ADJ assembles 250 four-input objects whose central
/// pair is visible and whose outer pair is adjacency-only context.
pub const LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT;
/// Key 3's alternate interpretation prepends and appends the adjacency-only
/// endpoint of the same 1,000-seed snake strip.
pub const LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT + 2;
/// Key 4's alternate interpretation supplies three edge neighbours for each
/// of the 1,872 grid triangles: six input vertices per triangle.
pub const LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT: usize = LINE_GRID_TRIANGLE_LIST_INDEX_COUNT * 2;
pub const LINE_GRID_TRIANGLE_LIST_ADJ_INDICES_PER_RGB_COLOR: usize =
    LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT / RGB_COLOR_COUNT;
/// One continuous snake strip covers each adjacent pair of grid rows. It
/// reuses the join vertices between row pairs, so it references the same
/// 1,000 seeded positions while emitting the entire surface.
pub const LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT: usize =
    (LINE_GRID_ROWS - 1) * LINE_GRID_COLUMNS * 2;
/// Key 5's alternate interpretation has the same visible strip vertices in
/// every even slot and supplies one adjacent-only source vertex in each odd
/// slot, as required by native TRISTRIP_ADJ.
pub const LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT: usize =
    LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT * 2;
/// A closed triangle fan has one hub, one visit to every other seed vertex,
/// and a final repeat of its first rim vertex.
pub const LINE_GRID_TRIANGLE_FAN_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT;
/// Key 6 cycles these independent fan sizes.
pub const TRIANGLE_FAN_VERTEX_COUNTS: [usize; 7] = [5, 10, 25, 50, 125, 250, 1000];
pub const TRIANGLE_FAN_CATALOGUE_INDEX_COUNT: usize = TRIANGLE_FAN_VERTEX_COUNTS
    .iter()
    .map(|&size| LINE_GRID_VERTEX_COUNT + LINE_GRID_VERTEX_COUNT / size - 1)
    .sum();
/// Key 7 partitions the plane into one hundred local 2×5 seed tiles. Each
/// tile is interpreted as one ten-index native triangle fan.
pub const TRIANGLE_FAN_MESH_FAN_COUNT: usize = LINE_GRID_VERTEX_COUNT / 10;
pub const TRIANGLE_FAN_MESH_INDICES_PER_DRAW: usize = 10;
pub const TRIANGLE_FAN_MESH_INDEX_COUNT: usize =
    TRIANGLE_FAN_MESH_FAN_COUNT * TRIANGLE_FAN_MESH_INDICES_PER_DRAW;
/// Key 8 reads the same seed plane as the other native-grid modes. Its
/// 39×24 cells are independent Intel QUADLIST primitives, split into two
/// equal opaque draws by checkerboard parity.
pub const QUAD_LIST_CELL_COUNT: usize = (LINE_GRID_COLUMNS - 1) * (LINE_GRID_ROWS - 1);
pub const QUAD_LIST_COLOR_DRAW_COUNT: usize = 2;
pub const QUAD_LIST_CELLS_PER_COLOR: usize = QUAD_LIST_CELL_COUNT / QUAD_LIST_COLOR_DRAW_COUNT;
pub const QUAD_LIST_INDICES_PER_DRAW: usize = QUAD_LIST_CELLS_PER_COLOR * 4;
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
pub const RECT_LIST_CELLS_PER_COLOR: usize = RECT_LIST_CELL_COUNT / RECT_LIST_COLOR_DRAW_COUNT;
pub const RECT_LIST_INDICES_PER_DRAW: usize =
    RECT_LIST_CELLS_PER_COLOR * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const RECT_LIST_INDEX_COUNT: usize = RECT_LIST_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const EXECUTION_VERTEX_COUNT: usize = RECT_LIST_VERTEX_OFFSET + RECT_LIST_VERTEX_COUNT;
pub const PRIMITIVE_MODE_COUNT: usize = 14;
pub const EXECUTION_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT * 3
    + LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT
    + LINE_GRID_TRIANGLE_LIST_INDEX_COUNT
    + LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT
    + LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT
    + LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT
    + TRIANGLE_FAN_CATALOGUE_INDEX_COUNT
    + TRIANGLE_FAN_MESH_INDEX_COUNT
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
    TriangleFanMesh10,
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
        Self::TriangleFanMesh10,
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
            Self::TriangleFanMesh10 => "triangle-fan-mesh-100x10",
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
            Self::TriangleList => LINE_GRID_TRIANGLE_LIST_INDICES_PER_RGB_COLOR,
            Self::TriangleListAdj => LINE_GRID_TRIANGLE_LIST_ADJ_INDICES_PER_RGB_COLOR,
            Self::TriangleStrip => LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT,
            Self::TriangleStripAdj => LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT,
            Self::TriangleFan => LINE_GRID_TRIANGLE_FAN_INDEX_COUNT,
            Self::TriangleFanMesh10 => TRIANGLE_FAN_MESH_INDICES_PER_DRAW,
            Self::QuadList => QUAD_LIST_INDICES_PER_DRAW,
            Self::QuadStrip => QUAD_STRIP_INDICES_PER_DRAW,
            Self::RectList => RECT_LIST_INDICES_PER_DRAW,
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
            Self::TriangleList | Self::TriangleListAdj => RGB_COLOR_COUNT,
            Self::TriangleFanMesh10 => TRIANGLE_FAN_MESH_FAN_COUNT,
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
            Self::TriangleFanMesh10 => 10,
            Self::QuadList => 11,
            Self::QuadStrip => 12,
            Self::RectList => 13,
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
            Self::TriangleFanMesh10 => 7,
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
            Self::TriangleFanMesh10 => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
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
            // The former ten-vertex interpretation is now part of key 6's
            // cycle; key 7 has no function.
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
    pub fan_first_indices: [u32; 7],
    pub fan_index_counts: [u32; 7],
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
            1
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
            } else {
                (
                    if mode == PrimitiveMode::TriangleFan {
                        self.fan_index_counts[TRIANGLE_FAN_VERTEX_COUNTS
                            .iter()
                            .position(|&size| size == fan_vertices)
                            .expect("invalid triangle fan size")]
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
                    if mode == PrimitiveMode::TriangleFanMesh10
                        || mode == PrimitiveMode::TriangleFan
                    {
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
                // The 100-fan Key 7 mesh deliberately cycles the opaque RGB
                // palette while all one-to-four draw modes retain their
                // authored per-draw palette selection.
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

    /// Build every native primitive interpretation once from the 1,000-vertex
    /// Picasso-seeded plane. Frame submission only selects ranges from this
    /// catalogue; it never rewrites execution-buffer bytes.
    pub fn execution_index_catalogue(&self) -> ExecutionIndexCatalogue {
        let mut indices = [0; EXECUTION_INDEX_COUNT];
        let mut first_indices = [0; PRIMITIVE_MODE_COUNT];
        let mut fan_first_indices = [0u32; 7];
        let mut fan_index_counts = [0u32; 7];
        let mut cursor = 0usize;
        for (mode_slot, mode) in PrimitiveMode::ALL.into_iter().enumerate() {
            if mode == PrimitiveMode::LineListAdj {
                // This is the defining PotatoStamps reinterpretation: Key 2's
                // second form changes only the topology. It reuses the exact
                // first index and 1,000-index count of ordinary LINELIST.
                first_indices[mode_slot] = first_indices[PrimitiveMode::LineList.slot()];
                continue;
            }
            first_indices[mode_slot] = cursor as u32;
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
            if mode == PrimitiveMode::LineStripAdj {
                // PRM: the first and last vertices are adjacent-only. Keep
                // the original 1,000-seed snake in the interior so the
                // visible line strip is identical to Key 3's ordinary form.
                let first = line_grid_snake_seed_vertex(0);
                let last = line_grid_snake_seed_vertex(LINE_GRID_VERTEX_COUNT - 1);
                indices[cursor] = line_grid_vertex(first);
                for step in 0..LINE_GRID_VERTEX_COUNT {
                    indices[cursor + 1 + step] =
                        line_grid_vertex(line_grid_snake_seed_vertex(step));
                }
                indices[cursor + LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT - 1] = line_grid_vertex(last);
                cursor += LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleList {
                // Two CCW triangles per cell. Pack their indices into three
                // contiguous draws by source-triangle ordinal, cycling
                // red -> green -> blue while preserving opaque per-draw color.
                let first_triangle_index = cursor;
                for color in 0..RGB_COLOR_COUNT {
                    let mut color_cursor = first_triangle_index
                        + color * LINE_GRID_TRIANGLE_LIST_INDICES_PER_RGB_COLOR;
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
                        first_triangle_index
                            + (color + 1) * LINE_GRID_TRIANGLE_LIST_INDICES_PER_RGB_COLOR
                    );
                }
                cursor += LINE_GRID_TRIANGLE_LIST_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleListAdj {
                // The even slots are the same CCW triangles as Key 4.  Each
                // odd slot supplies the vertex opposite that edge in its
                // neighbouring cell. At the outer boundary the triangle's
                // opposite vertex is repeated, yielding a degenerate
                // neighbour that makes the missing neighbour explicit.
                let first_triangle_index = cursor;
                for color in 0..RGB_COLOR_COUNT {
                    let mut color_cursor = first_triangle_index
                        + color * LINE_GRID_TRIANGLE_LIST_ADJ_INDICES_PER_RGB_COLOR;
                    for row in 0..LINE_GRID_ROWS - 1 {
                        for column in 0..LINE_GRID_COLUMNS - 1 {
                            let triangle = ((row * (LINE_GRID_COLUMNS - 1) + column) * 2) % 3;
                            if triangle == color {
                                indices[color_cursor..color_cursor + 6].copy_from_slice(
                                    &triangle_list_adjacency_vertices(row, column, 0),
                                );
                                color_cursor += 6;
                            }
                            if (triangle + 1) % 3 == color {
                                indices[color_cursor..color_cursor + 6].copy_from_slice(
                                    &triangle_list_adjacency_vertices(row, column, 1),
                                );
                                color_cursor += 6;
                            }
                        }
                    }
                    debug_assert_eq!(
                        color_cursor,
                        first_triangle_index
                            + (color + 1) * LINE_GRID_TRIANGLE_LIST_ADJ_INDICES_PER_RGB_COLOR
                    );
                }
                cursor += LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleStrip {
                // Each row pair is woven from the same seed positions. The
                // direction and top/bottom ordering alternate so every
                // non-degenerate strip triangle remains CCW. At a row join
                // the repeated edge positions form two degenerate triangles,
                // then the next 39×1 cell band starts without a new draw.
                for step in 0..LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT {
                    indices[cursor + step] = triangle_strip_main_vertex(step);
                }
                cursor += LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleStripAdj {
                // PRM: the even-numbered input vertices form the connected
                // strip; each odd-numbered input vertex is adjacency-only.
                // Reuse the exact Key 5 strip for the former.  The latter is
                // chosen from the neighbouring grid band; on the exterior,
                // the in-band mate is repeated as an explicit open boundary.
                for step in 0..LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT {
                    indices[cursor + step * 2] = triangle_strip_main_vertex(step);
                    indices[cursor + step * 2 + 1] = triangle_strip_adjacent_vertex(step);
                }
                cursor += LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT;
                continue;
            }
            if mode == PrimitiveMode::TriangleFan {
                // Key 6 reinterprets the fixed 1,000-index seed as
                // independent contiguous fans. The selected fan size is
                // applied at submission time; the catalogue stores the seed
                // ordinals once, so no vertices are duplicated or hidden.
                for (fan_index, &fan_size) in TRIANGLE_FAN_VERTEX_COUNTS.iter().enumerate() {
                    fan_first_indices[fan_index] = cursor as u32;
                    let fan_count = LINE_GRID_VERTEX_COUNT / fan_size;
                    let range_count = LINE_GRID_VERTEX_COUNT + fan_count - 1;
                    fan_index_counts[fan_index] = range_count as u32;
                    for vertex in 0..LINE_GRID_VERTEX_COUNT {
                        indices[cursor] = line_grid_vertex(vertex);
                        cursor += 1;
                        if vertex % fan_size == fan_size - 1 && vertex + 1 < LINE_GRID_VERTEX_COUNT
                        {
                            indices[cursor] = u32::MAX;
                            cursor += 1;
                        }
                    }
                }
                continue;
            }
            if mode == PrimitiveMode::TriangleFanMesh10 {
                // 20 two-column by 5-row tiles across, five tiles high:
                // 100 local fans, each with its own hub and nine rim seeds.
                // Every one of the 1,000 source vertices appears exactly once
                // in this fleet of ten-index triangle-fan primitives.
                for fan in 0..TRIANGLE_FAN_MESH_FAN_COUNT {
                    for vertex in 0..TRIANGLE_FAN_MESH_INDICES_PER_DRAW {
                        indices[cursor + vertex] =
                            line_grid_vertex(triangle_fan_mesh_seed_vertex(fan, vertex));
                    }
                    cursor += TRIANGLE_FAN_MESH_INDICES_PER_DRAW;
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
                    let mut color_cursor = first_quad_index + color * QUAD_LIST_INDICES_PER_DRAW;
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
                        first_quad_index + (color + 1) * QUAD_LIST_INDICES_PER_DRAW
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
                    let mut color_cursor = first_rect_index + color * RECT_LIST_INDICES_PER_DRAW;
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
                        first_rect_index + (color + 1) * RECT_LIST_INDICES_PER_DRAW
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

/// The snake order used by both LINESTRIP and LINESTRIP_ADJ.  Its source
/// range is exactly the existing 40×25, 1,000-vertex grid.
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

/// The odd TRISTRIP_ADJ slot corresponding to one ordinary strip vertex.
/// For interior row bands it selects the seed across the horizontal band edge.
/// At the outer grid boundary it duplicates the in-band mate, a degenerate
/// adjacent triangle that represents the absent neighbour without a sentinel
/// index or a synthetic vertex.
const fn triangle_strip_adjacent_vertex(step: usize) -> u32 {
    let vertices_per_band = LINE_GRID_COLUMNS * 2;
    let band_row = step / vertices_per_band;
    let source = triangle_strip_main_seed_vertex(step);
    let source_row = source / LINE_GRID_COLUMNS;
    let column = source % LINE_GRID_COLUMNS;
    let adjacent_row = if source_row == band_row {
        if band_row == 0 {
            band_row + 1
        } else {
            band_row - 1
        }
    } else if band_row + 2 < LINE_GRID_ROWS {
        band_row + 2
    } else {
        band_row
    };
    line_grid_vertex(adjacent_row * LINE_GRID_COLUMNS + column)
}

/// Return the six vertices of one native TRILIST_ADJ object in PRM order:
/// primary vertex, adjacent vertex, primary vertex, adjacent vertex, primary
/// vertex, adjacent vertex. The primary slots are therefore `[0, 2, 4]`.
const fn triangle_list_adjacency_vertices(row: usize, column: usize, half: usize) -> [u32; 6] {
    let lower_left = line_grid_vertex(row * LINE_GRID_COLUMNS + column);
    let lower_right = line_grid_vertex(row * LINE_GRID_COLUMNS + column + 1);
    let upper_right = line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column + 1);
    let upper_left = line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column);
    match half {
        // CCW lower-right triangle: lower-left, lower-right, upper-right.
        0 => [
            lower_left,
            if row == 0 {
                upper_right
            } else {
                line_grid_vertex((row - 1) * LINE_GRID_COLUMNS + column)
            },
            lower_right,
            if column + 1 == LINE_GRID_COLUMNS - 1 {
                lower_left
            } else {
                line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column + 2)
            },
            upper_right,
            upper_left,
        ],
        // CCW upper-left triangle: lower-left, upper-right, upper-left.
        1 => [
            lower_left,
            lower_right,
            upper_right,
            if row + 1 == LINE_GRID_ROWS - 1 {
                lower_left
            } else {
                line_grid_vertex((row + 2) * LINE_GRID_COLUMNS + column + 1)
            },
            upper_left,
            if column == 0 {
                upper_right
            } else {
                line_grid_vertex(row * LINE_GRID_COLUMNS + column - 1)
            },
        ],
        _ => [0; 6], // caller supplies the two cell halves, 0 and 1.
    }
}

const fn rect_list_vertex(rectangle: usize, corner: usize) -> u32 {
    (RECT_LIST_VERTEX_OFFSET + rectangle * RECT_LIST_VERTICES_PER_RECTANGLE + corner) as u32
}

/// Return one unique seed vertex for a ten-index local fan. The 2×5 tiles
/// exactly partition the 40×25 seed plane: 20 tiles across by five tiles high.
fn triangle_fan_mesh_seed_vertex(fan: usize, slot: usize) -> usize {
    let tile_columns = LINE_GRID_COLUMNS / 2;
    let tile_row = fan / tile_columns;
    let tile_column = fan % tile_columns;
    let row_base = tile_row * 5;
    let column_base = tile_column * 2;
    let (row_offset, column_offset) = match slot {
        0 => (2, 0), // hub
        1 => (2, 1),
        2 => (3, 1),
        3 => (4, 1),
        4 => (4, 0),
        5 => (3, 0),
        6 => (1, 0),
        7 => (0, 0),
        8 => (0, 1),
        9 => (1, 1),
        _ => unreachable!("triangle-fan mesh slot must be below ten"),
    };
    (row_base + row_offset) * LINE_GRID_COLUMNS + column_base + column_offset
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
mod tests {
    use super::*;

    #[test]
    fn authored_scene_is_four_ccw_triangles() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        assert_eq!(scene.positions.len(), VERTEX_COUNT);
        assert_eq!(scene.indices.len(), INDEX_COUNT);
        assert!(all_triangles_are_ccw(&scene.positions, &scene.indices));
    }

    #[test]
    fn native_quad_and_rectangle_modes_have_the_requested_hotkeys() {
        assert_eq!(PrimitiveMode::TriangleFan.number_key(), 6);
        assert_eq!(PrimitiveMode::TriangleFan.on_number_key_pressed(7), None);
        assert_eq!(PrimitiveMode::QuadList.number_key(), 8);
        assert_eq!(PrimitiveMode::QuadList.number_key_hid_usage(), 0x25);
        assert_eq!(PrimitiveMode::QuadStrip.number_key(), 9);
        assert_eq!(PrimitiveMode::QuadStrip.number_key_hid_usage(), 0x26);
        assert_eq!(PrimitiveMode::RectList.number_key(), 0);
        assert_eq!(PrimitiveMode::RectList.number_key_hid_usage(), 0x27);
    }

    #[test]
    fn palette_is_a_valid_opaque_bmp() {
        assert_eq!(&COLOR_TEXTURE_BYTES[..2], b"BM");
        assert_eq!(COLOR_TEXTURE_BYTES.len(), 70);
        assert_eq!(&COLOR_TEXTURE_BYTES[18..26], &[4, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(
            &COLOR_TEXTURE_BYTES[54..],
            &[
                0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 255, 255, 255
            ]
        );
        assert_eq!(
            decode_palette_rgba(COLOR_TEXTURE_BYTES).unwrap(),
            [
                u32::from_le_bytes([255, 0, 0, 255]),
                u32::from_le_bytes([0, 255, 0, 255]),
                u32::from_le_bytes([0, 0, 255, 255]),
                u32::from_le_bytes([255, 255, 0, 255]),
            ]
        );
    }

    #[test]
    fn execution_catalogue_contains_all_fourteen_native_topologies() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        assert_eq!(catalogue.indices.len(), EXECUTION_INDEX_COUNT);
        let point_first = catalogue.first_indices[PrimitiveMode::PointList.slot()] as usize;
        let line_list_first = catalogue.first_indices[PrimitiveMode::LineList.slot()] as usize;
        let line_list_adj_first =
            catalogue.first_indices[PrimitiveMode::LineListAdj.slot()] as usize;
        let line_strip_first = catalogue.first_indices[PrimitiveMode::LineStrip.slot()] as usize;
        let seed: [u32; LINE_GRID_VERTEX_COUNT] = core::array::from_fn(line_grid_vertex);
        let point_indices = &catalogue.indices[point_first..point_first + LINE_GRID_VERTEX_COUNT];
        let mut point_seen = [false; LINE_GRID_VERTEX_COUNT];
        for (ordinal, &index) in point_indices[..POINT_GRID_GREEN_INDEX_COUNT]
            .iter()
            .enumerate()
        {
            let source_ordinal = index as usize - LINE_GRID_VERTEX_OFFSET;
            assert_eq!(source_ordinal, ordinal * RGB_COLOR_COUNT);
            point_seen[source_ordinal] = true;
        }
        for &index in &point_indices[POINT_GRID_GREEN_INDEX_COUNT..] {
            let source_ordinal = index as usize - LINE_GRID_VERTEX_OFFSET;
            assert_ne!(source_ordinal % RGB_COLOR_COUNT, 0);
            assert!(!point_seen[source_ordinal]);
            point_seen[source_ordinal] = true;
        }
        assert!(point_seen.into_iter().all(|used| used));
        assert_eq!(
            &catalogue.indices[line_list_first..line_list_first + LINE_GRID_VERTEX_COUNT],
            &seed
        );
        assert_eq!(line_list_adj_first, line_list_first);

        let line_strip =
            &catalogue.indices[line_strip_first..line_strip_first + LINE_GRID_VERTEX_COUNT];
        let mut seen = [false; LINE_GRID_VERTEX_COUNT];
        for &index in line_strip {
            let local = index as usize - LINE_GRID_VERTEX_OFFSET;
            assert!(local < LINE_GRID_VERTEX_COUNT);
            assert!(!seen[local]);
            seen[local] = true;
        }
        assert!(seen.into_iter().all(|used| used));
        assert_eq!(line_strip[..LINE_GRID_COLUMNS], seed[..LINE_GRID_COLUMNS]);
        for step in 0..LINE_GRID_COLUMNS {
            assert_eq!(
                line_strip[LINE_GRID_COLUMNS + step],
                seed[LINE_GRID_COLUMNS * 2 - 1 - step]
            );
        }

        let triangle_list_first =
            catalogue.first_indices[PrimitiveMode::TriangleList.slot()] as usize;
        let triangle_list = &catalogue.indices
            [triangle_list_first..triangle_list_first + LINE_GRID_TRIANGLE_LIST_INDEX_COUNT];
        assert_eq!(
            &triangle_list[..6],
            &[
                line_grid_vertex(0),
                line_grid_vertex(1),
                line_grid_vertex(LINE_GRID_COLUMNS + 1),
                line_grid_vertex(1),
                line_grid_vertex(LINE_GRID_COLUMNS + 2),
                line_grid_vertex(LINE_GRID_COLUMNS + 1),
            ]
        );
        let triangle_fan_first =
            catalogue.first_indices[PrimitiveMode::TriangleFan.slot()] as usize;
        assert_eq!(
            &catalogue.indices[triangle_fan_first..triangle_fan_first + 3],
            &[
                line_grid_vertex(0),
                line_grid_vertex(1),
                line_grid_vertex(2)
            ]
        );
        let triangle_fan_mesh_first =
            catalogue.first_indices[PrimitiveMode::TriangleFanMesh10.slot()] as usize;
        assert_eq!(
            &catalogue.indices[triangle_fan_mesh_first
                ..triangle_fan_mesh_first + TRIANGLE_FAN_MESH_INDICES_PER_DRAW],
            &[
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 0)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 1)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 2)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 3)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 4)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 5)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 6)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 7)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 8)),
                line_grid_vertex(triangle_fan_mesh_seed_vertex(0, 9)),
            ]
        );
        let triangle_strip_first =
            catalogue.first_indices[PrimitiveMode::TriangleStrip.slot()] as usize;
        assert_eq!(
            &catalogue.indices[triangle_strip_first..triangle_strip_first + 6],
            &[
                line_grid_vertex(LINE_GRID_COLUMNS),
                line_grid_vertex(0),
                line_grid_vertex(LINE_GRID_COLUMNS + 1),
                line_grid_vertex(1),
                line_grid_vertex(LINE_GRID_COLUMNS + 2),
                line_grid_vertex(2),
            ]
        );
        let quad_first = catalogue.first_indices[PrimitiveMode::QuadList.slot()] as usize;
        assert_eq!(
            &catalogue.indices[quad_first..quad_first + 8],
            &[
                line_grid_vertex(0),
                line_grid_vertex(1),
                line_grid_vertex(LINE_GRID_COLUMNS + 1),
                line_grid_vertex(LINE_GRID_COLUMNS),
                line_grid_vertex(2),
                line_grid_vertex(3),
                line_grid_vertex(LINE_GRID_COLUMNS + 3),
                line_grid_vertex(LINE_GRID_COLUMNS + 2),
            ]
        );
        let quad_strip_first = catalogue.first_indices[PrimitiveMode::QuadStrip.slot()] as usize;
        assert_eq!(
            &catalogue.indices[quad_strip_first..quad_strip_first + QUAD_STRIP_INDICES_PER_DRAW],
            &[17, 12, 18, 13, 19, 14, 20, 15, 21, 16]
        );
        let rect_first = catalogue.first_indices[PrimitiveMode::RectList.slot()] as usize;
        assert_eq!(
            &catalogue.indices[rect_first..rect_first + 6],
            &[
                rect_list_vertex(0, 0),
                rect_list_vertex(0, 1),
                rect_list_vertex(0, 2),
                rect_list_vertex(2, 0),
                rect_list_vertex(2, 1),
                rect_list_vertex(2, 2),
            ]
        );
        assert!(
            catalogue
                .indices
                .into_iter()
                .all(|index| index == u32::MAX || index < EXECUTION_VERTEX_COUNT as u32)
        );
    }

    #[test]
    fn line_seed_is_an_evenly_distributed_thousand_vertex_plane() {
        let grid = line_grid_positions();
        assert_eq!(grid.len(), 1_000);
        assert!(grid.iter().all(|position| position.z == 0.0));
        assert_eq!(grid[0].x, -0.95);
        assert_eq!(grid[0].y, -0.95);
        assert!((grid[LINE_GRID_COLUMNS - 1].x - 0.95).abs() < 1e-6);
        assert!((grid[LINE_GRID_VERTEX_COUNT - 1].x - 0.95).abs() < 1e-6);
        assert!((grid[LINE_GRID_VERTEX_COUNT - 1].y - 0.95).abs() < 1e-6);
        assert!(
            ((grid[1].x - grid[0].x) - (grid[LINE_GRID_COLUMNS + 1].x - grid[LINE_GRID_COLUMNS].x))
                .abs()
                < 1e-6
        );
        assert!(
            ((grid[LINE_GRID_COLUMNS].y - grid[0].y)
                - (grid[LINE_GRID_COLUMNS * 2].y - grid[LINE_GRID_COLUMNS].y))
                .abs()
                < 1e-6
        );
    }

    #[test]
    fn triangle_list_tessellates_the_same_line_seed_with_ccw_triangles() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::TriangleList.slot()] as usize;
        let triangle_indices =
            &catalogue.indices[first..first + LINE_GRID_TRIANGLE_LIST_INDEX_COUNT];
        assert_eq!(
            LINE_GRID_TRIANGLE_LIST_INDEX_COUNT % RGB_COLOR_COUNT,
            0,
            "the RGB loop needs equal index ranges"
        );
        assert_eq!(triangle_indices.len() % 3, 0);
        assert_eq!(
            triangle_indices.len() / 3,
            (LINE_GRID_COLUMNS - 1) * (LINE_GRID_ROWS - 1) * 2
        );

        let grid = line_grid_positions();
        let mut used = [false; LINE_GRID_VERTEX_COUNT];
        for triangle in triangle_indices.chunks_exact(3) {
            let local = [
                triangle[0] as usize - LINE_GRID_VERTEX_OFFSET,
                triangle[1] as usize - LINE_GRID_VERTEX_OFFSET,
                triangle[2] as usize - LINE_GRID_VERTEX_OFFSET,
            ];
            for &index in &local {
                used[index] = true;
            }
            let a = grid[local[0]];
            let b = grid[local[1]];
            let c = grid[local[2]];
            assert!((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) > 0.0);
        }
        assert!(used.into_iter().all(|vertex| vertex));

        let colors = [0xff00_00ff, 0xff00_ff00, 0xffff_0000, 0xffff_ffff];
        let batch = catalogue.draw_batch(PrimitiveMode::TriangleList, colors, 0);
        assert_eq!(batch.draw_count, RGB_COLOR_COUNT as u32);
        for color in 0..RGB_COLOR_COUNT {
            assert_eq!(
                batch.draws[color].index_count,
                LINE_GRID_TRIANGLE_LIST_INDICES_PER_RGB_COLOR as u32
            );
            assert_eq!(batch.draws[color].rgba8_srgb, colors[color]);
            assert_eq!(batch.draws[color].rgba8_srgb >> 24, u8::MAX as u32);
        }
    }

    #[test]
    fn triangle_strip_tessellates_the_same_thousand_seed_vertices() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::TriangleStrip.slot()] as usize;
        let strip = &catalogue.indices[first..first + LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT];
        assert_eq!(strip.len(), (LINE_GRID_ROWS - 1) * LINE_GRID_COLUMNS * 2);

        let grid = line_grid_positions();
        let mut used = [false; LINE_GRID_VERTEX_COUNT];
        for &index in strip {
            used[index as usize - LINE_GRID_VERTEX_OFFSET] = true;
        }
        assert!(used.into_iter().all(|vertex| vertex));

        for triangle_start in 0..strip.len() - 2 {
            let [a, b, c] = if triangle_start.is_multiple_of(2) {
                [
                    strip[triangle_start],
                    strip[triangle_start + 1],
                    strip[triangle_start + 2],
                ]
            } else {
                [
                    strip[triangle_start + 1],
                    strip[triangle_start],
                    strip[triangle_start + 2],
                ]
            };
            let a = grid[a as usize - LINE_GRID_VERTEX_OFFSET];
            let b = grid[b as usize - LINE_GRID_VERTEX_OFFSET];
            let c = grid[c as usize - LINE_GRID_VERTEX_OFFSET];
            let twice_area = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
            assert!(twice_area >= 0.0, "triangle {triangle_start} is clockwise");
        }
    }

    #[test]
    fn keys_two_to_five_toggle_native_adjacency_over_the_same_seed_plane() {
        let pairs = [
            (
                PrimitiveMode::LineList,
                PrimitiveMode::LineListAdj,
                2,
                trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST_ADJ,
            ),
            (
                PrimitiveMode::LineStrip,
                PrimitiveMode::LineStripAdj,
                3,
                trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_STRIP_ADJ,
            ),
            (
                PrimitiveMode::TriangleList,
                PrimitiveMode::TriangleListAdj,
                4,
                trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_ADJ,
            ),
            (
                PrimitiveMode::TriangleStrip,
                PrimitiveMode::TriangleStripAdj,
                5,
                trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_ADJ,
            ),
        ];
        for (ordinary, adjacent, key, topology) in pairs {
            assert_eq!(ordinary.number_key(), key);
            assert_eq!(adjacent.number_key(), key);
            assert_eq!(ordinary.number_key_hid_usage(), 0x1d + key);
            assert_eq!(ordinary.on_number_key_pressed(key), Some(adjacent));
            assert_eq!(adjacent.on_number_key_pressed(key), Some(ordinary));
            assert_eq!(adjacent.vgpu_topology(), topology);
            assert!(!ordinary.requires_adjacency_topology_rendering());
            assert!(adjacent.requires_adjacency_topology_rendering());
        }
        assert_eq!(
            PrimitiveMode::TriangleFan.on_number_key_pressed(4),
            Some(PrimitiveMode::TriangleList)
        );
    }

    #[test]
    fn key_two_line_list_toggle_changes_only_the_topology_word() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let colors = [0x11, 0x22, 0x33, 0x44];
        let ordinary = catalogue
            .draw_batch(PrimitiveMode::LineList, colors, 0)
            .draws[0];
        let adjacent = catalogue
            .draw_batch(PrimitiveMode::LineListAdj, colors, 0)
            .draws[0];

        let mut expected = ordinary;
        expected.topology = trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST_ADJ;
        assert_eq!(adjacent, expected);
        assert_eq!(ordinary.index_count as usize / 2, 500);
        assert_eq!(adjacent.index_count as usize / 4, 250);
    }

    #[test]
    fn adjacency_catalogues_obey_their_native_input_contracts() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let seed_start = LINE_GRID_VERTEX_OFFSET as u32;
        let seed_end = seed_start + LINE_GRID_VERTEX_COUNT as u32;

        let line_list_first = catalogue.first_indices[PrimitiveMode::LineList.slot()] as usize;
        let line_list =
            &catalogue.indices[line_list_first..line_list_first + LINE_GRID_VERTEX_COUNT];
        let line_list_adj_first =
            catalogue.first_indices[PrimitiveMode::LineListAdj.slot()] as usize;
        let line_list_adj = &catalogue.indices
            [line_list_adj_first..line_list_adj_first + LINE_GRID_LINE_LIST_ADJ_INDEX_COUNT];
        assert_eq!(line_list_adj_first, line_list_first);
        assert_eq!(line_list_adj, line_list);
        assert_eq!(line_list.len() / 2, 500);
        assert_eq!(line_list_adj.len() / 4, 250);
        for adjacency in line_list_adj.chunks_exact(4) {
            assert!(
                adjacency
                    .iter()
                    .all(|&index| index >= seed_start && index < seed_end)
            );
        }

        let strip_first = catalogue.first_indices[PrimitiveMode::LineStrip.slot()] as usize;
        let strip = &catalogue.indices[strip_first..strip_first + LINE_GRID_VERTEX_COUNT];
        let strip_adj_first = catalogue.first_indices[PrimitiveMode::LineStripAdj.slot()] as usize;
        let strip_adj = &catalogue.indices
            [strip_adj_first..strip_adj_first + LINE_GRID_LINE_STRIP_ADJ_INDEX_COUNT];
        assert_eq!(&strip_adj[1..strip_adj.len() - 1], strip);
        assert_eq!(strip_adj[0], strip[0]);
        assert_eq!(strip_adj[strip_adj.len() - 1], strip[strip.len() - 1]);

        let triangle_list_first =
            catalogue.first_indices[PrimitiveMode::TriangleList.slot()] as usize;
        let triangle_list = &catalogue.indices
            [triangle_list_first..triangle_list_first + LINE_GRID_TRIANGLE_LIST_INDEX_COUNT];
        let triangle_list_adj_first =
            catalogue.first_indices[PrimitiveMode::TriangleListAdj.slot()] as usize;
        let triangle_list_adj = &catalogue.indices[triangle_list_adj_first
            ..triangle_list_adj_first + LINE_GRID_TRIANGLE_LIST_ADJ_INDEX_COUNT];
        for (ordinary, adjacency) in triangle_list
            .chunks_exact(3)
            .zip(triangle_list_adj.chunks_exact(6))
        {
            assert_eq!([adjacency[0], adjacency[2], adjacency[4]], ordinary);
            assert!(
                adjacency
                    .iter()
                    .all(|&index| index >= seed_start && index < seed_end)
            );
        }

        let triangle_strip_first =
            catalogue.first_indices[PrimitiveMode::TriangleStrip.slot()] as usize;
        let triangle_strip = &catalogue.indices
            [triangle_strip_first..triangle_strip_first + LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT];
        let triangle_strip_adj_first =
            catalogue.first_indices[PrimitiveMode::TriangleStripAdj.slot()] as usize;
        let triangle_strip_adj = &catalogue.indices[triangle_strip_adj_first
            ..triangle_strip_adj_first + LINE_GRID_TRIANGLE_STRIP_ADJ_INDEX_COUNT];
        for (ordinary, adjacency) in triangle_strip
            .iter()
            .zip(triangle_strip_adj.chunks_exact(2))
        {
            assert_eq!(adjacency[0], *ordinary);
            assert!(adjacency[1] >= seed_start && adjacency[1] < seed_end);
        }

        let colors = [0xff00_00ff, 0xff00_ff00, 0xffff_0000, 0xffff_ffff];
        for mode in [
            PrimitiveMode::LineListAdj,
            PrimitiveMode::LineStripAdj,
            PrimitiveMode::TriangleListAdj,
            PrimitiveMode::TriangleStripAdj,
        ] {
            let batch = catalogue.draw_batch(mode, colors, 0);
            assert!(
                batch.draws[..mode.draw_count()]
                    .iter()
                    .all(|draw| draw.topology == mode.vgpu_topology())
            );
        }
    }

    #[test]
    fn triangle_fan_partitions_all_thousand_seed_vertices() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::TriangleFan.slot()] as usize;
        let fan = &catalogue.indices[first..first + catalogue.fan_index_counts[0] as usize];
        assert_eq!(fan[0], line_grid_vertex(0));
        assert_eq!(
            fan[fan.len() - 1],
            line_grid_vertex(LINE_GRID_VERTEX_COUNT - 1)
        );
        let mut used = [false; LINE_GRID_VERTEX_COUNT];
        for &index in fan {
            if index != u32::MAX {
                used[index as usize - LINE_GRID_VERTEX_OFFSET] = true;
            }
        }
        assert!(used.into_iter().all(|vertex| vertex));

        for &size in &TRIANGLE_FAN_VERTEX_COUNTS {
            let batch =
                catalogue.draw_batch_with_fan_size(PrimitiveMode::TriangleFan, [0; 4], 0, size);
            assert_eq!(batch.draw_count, 1);
            let fan_index = TRIANGLE_FAN_VERTEX_COUNTS
                .iter()
                .position(|&value| value == size)
                .unwrap();
            let start = catalogue.fan_first_indices[fan_index] as usize;
            let range = &catalogue.indices[start..start + batch.draws[0].index_count as usize];
            assert_eq!(
                range.iter().filter(|&&index| index == u32::MAX).count(),
                LINE_GRID_VERTEX_COUNT / size - 1
            );
            let mut seen = [false; LINE_GRID_VERTEX_COUNT];
            for &index in range {
                if index != u32::MAX {
                    let local = index as usize - LINE_GRID_VERTEX_OFFSET;
                    assert!(!seen[local]);
                    seen[local] = true;
                }
            }
            assert!(seen.into_iter().all(|vertex| vertex));
            assert_eq!(
                batch.draws[0].index_count as usize,
                LINE_GRID_VERTEX_COUNT + LINE_GRID_VERTEX_COUNT / size - 1
            );
        }
    }

    #[test]
    fn key_seven_is_a_fleet_of_one_hundred_ten_seed_triangle_fans() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::TriangleFanMesh10.slot()] as usize;
        let mesh = &catalogue.indices[first..first + TRIANGLE_FAN_MESH_INDEX_COUNT];
        assert_eq!(mesh.len(), LINE_GRID_VERTEX_COUNT);
        assert_eq!(
            mesh.chunks_exact(TRIANGLE_FAN_MESH_INDICES_PER_DRAW).len(),
            100
        );

        let grid = line_grid_positions();
        let mut used = [false; LINE_GRID_VERTEX_COUNT];
        for (fan_index, fan) in mesh
            .chunks_exact(TRIANGLE_FAN_MESH_INDICES_PER_DRAW)
            .enumerate()
        {
            for (slot, &index) in fan.iter().enumerate() {
                assert_eq!(
                    index,
                    line_grid_vertex(triangle_fan_mesh_seed_vertex(fan_index, slot))
                );
                let local = index as usize - LINE_GRID_VERTEX_OFFSET;
                assert!(!used[local]);
                used[local] = true;
            }
            let hub = grid[fan[0] as usize - LINE_GRID_VERTEX_OFFSET];
            for rim in fan[1..].windows(2) {
                let b = grid[rim[0] as usize - LINE_GRID_VERTEX_OFFSET];
                let c = grid[rim[1] as usize - LINE_GRID_VERTEX_OFFSET];
                let twice_area = (b.x - hub.x) * (c.y - hub.y) - (b.y - hub.y) * (c.x - hub.x);
                assert!(twice_area >= -1e-6, "fan {fan_index} has a clockwise wedge");
            }
        }
        assert!(used.into_iter().all(|vertex| vertex));

        let colors = [0xff00_00ff, 0xff00_ff00, 0xffff_0000, 0xffff_ffff];
        let batch = catalogue.draw_batch(PrimitiveMode::TriangleFanMesh10, colors, 0);
        assert_eq!(batch.draw_count, TRIANGLE_FAN_MESH_FAN_COUNT as u32);
        for draw in 0..TRIANGLE_FAN_MESH_FAN_COUNT {
            assert_eq!(batch.draws[draw].rgba8_srgb, colors[draw % RGB_COLOR_COUNT]);
            assert_eq!(batch.draws[draw].rgba8_srgb >> 24, u8::MAX as u32);
        }
    }

    #[test]
    fn quad_list_is_a_two_draw_checkerboard_over_the_same_thousand_seed_vertices() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::QuadList.slot()] as usize;
        let quad_indices = &catalogue.indices[first..first + QUAD_LIST_INDEX_COUNT];
        assert_eq!(QUAD_LIST_CELL_COUNT, 936);
        assert_eq!(QUAD_LIST_CELLS_PER_COLOR, 468);
        assert_eq!(quad_indices.len() % 4, 0);
        assert_eq!(quad_indices.len() / 4, QUAD_LIST_CELL_COUNT);

        let grid = line_grid_positions();
        let mut seen_cells = [false; QUAD_LIST_CELL_COUNT];
        let mut used_vertices = [false; LINE_GRID_VERTEX_COUNT];
        for color in 0..QUAD_LIST_COLOR_DRAW_COUNT {
            let quads = &quad_indices
                [color * QUAD_LIST_INDICES_PER_DRAW..(color + 1) * QUAD_LIST_INDICES_PER_DRAW];
            assert_eq!(quads.len() / 4, QUAD_LIST_CELLS_PER_COLOR);
            for quad in quads.chunks_exact(4) {
                let lower_left = quad[0] as usize - LINE_GRID_VERTEX_OFFSET;
                let row = lower_left / LINE_GRID_COLUMNS;
                let column = lower_left % LINE_GRID_COLUMNS;
                assert!(row < LINE_GRID_ROWS - 1);
                assert!(column < LINE_GRID_COLUMNS - 1);
                assert_eq!((row + column) % QUAD_LIST_COLOR_DRAW_COUNT, color);
                assert_eq!(
                    quad[1],
                    line_grid_vertex(row * LINE_GRID_COLUMNS + column + 1)
                );
                assert_eq!(
                    quad[2],
                    line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column + 1)
                );
                assert_eq!(
                    quad[3],
                    line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column)
                );
                let cell = row * (LINE_GRID_COLUMNS - 1) + column;
                assert!(!seen_cells[cell]);
                seen_cells[cell] = true;
                for &index in quad {
                    used_vertices[index as usize - LINE_GRID_VERTEX_OFFSET] = true;
                }
                let a = grid[lower_left];
                let b = grid[quad[1] as usize - LINE_GRID_VERTEX_OFFSET];
                let c = grid[quad[2] as usize - LINE_GRID_VERTEX_OFFSET];
                assert!((b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) > 0.0);
            }
        }
        assert!(seen_cells.into_iter().all(|seen| seen));
        assert!(used_vertices.into_iter().all(|used| used));

        let colors = [0xff00_00ff, 0xff00_ff00, 0xffff_0000, 0xffff_ffff];
        let batch = catalogue.draw_batch(PrimitiveMode::QuadList, colors, 0);
        assert_eq!(batch.draw_count, QUAD_LIST_COLOR_DRAW_COUNT as u32);
        for color in 0..QUAD_LIST_COLOR_DRAW_COUNT {
            assert_eq!(
                batch.draws[color].topology,
                trueos::vgpu::PRIMITIVE_TOPOLOGY_QUAD_LIST
            );
            assert_eq!(
                batch.draws[color].index_count,
                QUAD_LIST_INDICES_PER_DRAW as u32
            );
            assert_eq!(batch.draws[color].rgba8_srgb, colors[color]);
            assert_eq!(batch.draws[color].rgba8_srgb >> 24, u8::MAX as u32);
        }
    }

    #[test]
    fn quad_strip_is_a_shared_vertex_four_by_four_grid_on_key_nine() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::QuadStrip.slot()] as usize;

        for row in 0..QUAD_GRID_SIDE_QUADS {
            let strip = &catalogue.indices[first + row * QUAD_STRIP_INDICES_PER_DRAW
                ..first + (row + 1) * QUAD_STRIP_INDICES_PER_DRAW];
            for column in 0..QUAD_GRID_SIDE_VERTICES {
                assert_eq!(strip[column * 2], quad_grid_vertex(row + 1, column));
                assert_eq!(strip[column * 2 + 1], quad_grid_vertex(row, column));
            }
        }

        let grid = quad_grid_positions();
        assert_eq!(grid[0].x, -0.8);
        assert_eq!(grid[0].y, -0.8);
        assert_eq!(grid[QUAD_GRID_VERTEX_COUNT - 1].x, 0.8);
        assert_eq!(grid[QUAD_GRID_VERTEX_COUNT - 1].y, 0.8);
        let batch = catalogue.draw_batch(
            PrimitiveMode::QuadStrip,
            [0xff00_00ff, 0xff00_ff00, 0xffff_0000, 0xffff_ffff],
            0,
        );
        assert_eq!(batch.draw_count, QUAD_STRIP_DRAW_COUNT as u32);
        assert!(
            batch.draws[..QUAD_STRIP_DRAW_COUNT]
                .iter()
                .all(|draw| draw.topology == trueos::vgpu::PRIMITIVE_TOPOLOGY_QUAD_STRIP)
        );
    }

    #[test]
    fn rect_list_is_a_screen_space_two_draw_checkerboard_on_key_zero() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::RectList.slot()] as usize;
        let rect_indices = &catalogue.indices[first..first + RECT_LIST_INDEX_COUNT];
        assert_eq!(RECT_LIST_CELL_COUNT, 936);
        assert_eq!(RECT_LIST_CELLS_PER_COLOR, 468);
        assert_eq!(rect_indices.len() / 3, RECT_LIST_CELL_COUNT);

        let positions = rect_list_positions();
        let mut seen_rectangles = [false; RECT_LIST_CELL_COUNT];
        for color in 0..RECT_LIST_COLOR_DRAW_COUNT {
            let rectangles = &rect_indices
                [color * RECT_LIST_INDICES_PER_DRAW..(color + 1) * RECT_LIST_INDICES_PER_DRAW];
            assert_eq!(rectangles.len() / 3, RECT_LIST_CELLS_PER_COLOR);
            for indices in rectangles.chunks_exact(3) {
                let lower_right = indices[0] as usize - RECT_LIST_VERTEX_OFFSET;
                let lower_left = indices[1] as usize - RECT_LIST_VERTEX_OFFSET;
                let upper_left = indices[2] as usize - RECT_LIST_VERTEX_OFFSET;
                let rectangle = lower_right / RECT_LIST_VERTICES_PER_RECTANGLE;
                assert_eq!(lower_right % RECT_LIST_VERTICES_PER_RECTANGLE, 0);
                assert_eq!(lower_left, lower_right + 1);
                assert_eq!(upper_left, lower_right + 2);
                let row = rectangle / (LINE_GRID_COLUMNS - 1);
                let column = rectangle % (LINE_GRID_COLUMNS - 1);
                assert_eq!((row + column) % RECT_LIST_COLOR_DRAW_COUNT, color);
                assert!(!seen_rectangles[rectangle]);
                seen_rectangles[rectangle] = true;

                let v0 = positions[lower_right];
                let v1 = positions[lower_left];
                let v2 = positions[upper_left];
                let implied_upper_right = (v0.x - v1.x + v2.x, v0.y - v1.y + v2.y);
                assert!((implied_upper_right.0 - v0.x).abs() < 1e-6);
                assert!((implied_upper_right.1 - v2.y).abs() < 1e-6);
                assert!(v0.x > v1.x && v1.y > v2.y);
            }
        }
        assert!(seen_rectangles.into_iter().all(|seen| seen));
        let batch = catalogue.draw_batch(
            PrimitiveMode::RectList,
            [0xff00_00ff, 0xff00_ff00, 0xffff_0000, 0xffff_ffff],
            0,
        );
        assert_eq!(batch.draw_count, RECT_LIST_COLOR_DRAW_COUNT as u32);
        assert!(
            batch.draws[..RECT_LIST_COLOR_DRAW_COUNT]
                .iter()
                .all(|draw| draw.topology == trueos::vgpu::PRIMITIVE_TOPOLOGY_RECT_LIST)
        );
    }

    #[test]
    fn every_mode_lowers_to_exact_v2_draw_descriptors() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let colors = [0x11, 0x22, 0x33, 0x44];

        for mode in PrimitiveMode::ALL {
            let batch = catalogue.draw_batch(mode, colors, 0xaabb_ccdd);
            assert_eq!(batch.clear_rgba8_srgb, 0xaabb_ccdd);
            assert_eq!(batch.draw_count, mode.draw_count() as u32);
            for (draw_index, draw) in batch.draws[..mode.draw_count()].iter().enumerate() {
                assert_eq!(draw.topology, mode.vgpu_topology());
                let (expected_index_count, expected_first_index, expected_color) =
                    if mode == PrimitiveMode::PointList {
                        if draw_index == 0 {
                            (
                                POINT_GRID_GREEN_INDEX_COUNT as u32,
                                catalogue.first_indices[mode.slot()],
                                colors[1],
                            )
                        } else {
                            (
                                POINT_GRID_RED_INDEX_COUNT as u32,
                                catalogue.first_indices[mode.slot()]
                                    + POINT_GRID_GREEN_INDEX_COUNT as u32,
                                colors[0],
                            )
                        }
                    } else {
                        (
                            if mode == PrimitiveMode::TriangleFan {
                                catalogue.fan_index_counts[6]
                            } else {
                                mode.indices_per_draw() as u32
                            },
                            if mode == PrimitiveMode::TriangleFan {
                                catalogue.fan_first_indices[6]
                            } else {
                                catalogue.first_indices[mode.slot()]
                                    + draw_index as u32 * mode.indices_per_draw() as u32
                            },
                            colors[if mode == PrimitiveMode::TriangleFanMesh10
                                || mode == PrimitiveMode::TriangleFan
                            {
                                draw_index % RGB_COLOR_COUNT
                            } else {
                                draw_index
                            }],
                        )
                    };
                assert_eq!(draw.index_count, expected_index_count);
                assert_eq!(draw.first_index, expected_first_index);
                assert_eq!(draw.base_vertex, 0);
                assert_eq!(draw.rgba8_srgb, expected_color);
                assert_eq!(draw.reserved, 0);
            }
            for draw in &batch.draws[mode.draw_count()..] {
                assert_eq!(draw.index_count, 0);
                assert_eq!(draw.topology, 0);
                assert_eq!(draw.reserved, 0);
            }
        }
    }
}
