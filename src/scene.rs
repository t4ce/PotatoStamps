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
/// One thousand equally spaced source vertices. The three line-family modes
/// select from this exact range rather than inventing topology-specific
/// positions.
pub const LINE_GRID_COLUMNS: usize = 40;
pub const LINE_GRID_ROWS: usize = 25;
pub const LINE_GRID_VERTEX_COUNT: usize = LINE_GRID_COLUMNS * LINE_GRID_ROWS;
pub const LINE_GRID_VERTEX_OFFSET: usize = VERTEX_COUNT;
pub const LINE_GRID_XYZ_BYTES: usize = LINE_GRID_VERTEX_COUNT * 12;
pub const RGB_COLOR_COUNT: usize = 3;
pub const LINE_GRID_TRIANGLE_LIST_INDEX_COUNT: usize =
    (LINE_GRID_COLUMNS - 1) * (LINE_GRID_ROWS - 1) * 6;
pub const LINE_GRID_TRIANGLE_LIST_INDICES_PER_RGB_COLOR: usize =
    LINE_GRID_TRIANGLE_LIST_INDEX_COUNT / RGB_COLOR_COUNT;
/// One continuous snake strip covers each adjacent pair of grid rows. It
/// reuses the join vertices between row pairs, so it references the same
/// 1,000 seeded positions while emitting the entire surface.
pub const LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT: usize =
    (LINE_GRID_ROWS - 1) * LINE_GRID_COLUMNS * 2;
/// A closed triangle fan has one hub, one visit to every other seed vertex,
/// and a final repeat of its first rim vertex.
pub const LINE_GRID_TRIANGLE_FAN_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT + 1;
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
pub const EXECUTION_VERTEX_COUNT: usize = LINE_GRID_VERTEX_OFFSET + LINE_GRID_VERTEX_COUNT;
pub const PRIMITIVE_MODE_COUNT: usize = 8;
pub const EXECUTION_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT * 3
    + LINE_GRID_TRIANGLE_LIST_INDEX_COUNT
    + LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT
    + LINE_GRID_TRIANGLE_FAN_INDEX_COUNT
    + TRIANGLE_FAN_MESH_INDEX_COUNT
    + QUAD_LIST_INDEX_COUNT;

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
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    TriangleFanMesh10,
    QuadList,
}

impl PrimitiveMode {
    pub const ALL: [Self; PRIMITIVE_MODE_COUNT] = [
        Self::PointList,
        Self::LineList,
        Self::LineStrip,
        Self::TriangleList,
        Self::TriangleStrip,
        Self::TriangleFan,
        Self::TriangleFanMesh10,
        Self::QuadList,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PointList => "point-list-grid",
            Self::LineList => "line-list-grid",
            Self::LineStrip => "line-strip-grid",
            Self::TriangleList => "triangle-list-grid",
            Self::TriangleStrip => "triangle-strip",
            Self::TriangleFan => "triangle-fan",
            Self::TriangleFanMesh10 => "triangle-fan-mesh-100x10",
            Self::QuadList => "quad-list-checkerboard-grid",
        }
    }

    pub const fn indices_per_draw(self) -> usize {
        match self {
            Self::PointList | Self::LineList | Self::LineStrip => LINE_GRID_VERTEX_COUNT,
            Self::TriangleList => LINE_GRID_TRIANGLE_LIST_INDICES_PER_RGB_COLOR,
            Self::TriangleStrip => LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT,
            Self::TriangleFan => LINE_GRID_TRIANGLE_FAN_INDEX_COUNT,
            Self::TriangleFanMesh10 => TRIANGLE_FAN_MESH_INDICES_PER_DRAW,
            Self::QuadList => QUAD_LIST_INDICES_PER_DRAW,
        }
    }

    pub const fn draw_count(self) -> usize {
        match self {
            Self::PointList
            | Self::LineList
            | Self::LineStrip
            | Self::TriangleStrip
            | Self::TriangleFan => 1,
            // Three opaque constant-color draws: red, green, then blue.
            Self::TriangleList => RGB_COLOR_COUNT,
            Self::TriangleFanMesh10 => TRIANGLE_FAN_MESH_FAN_COUNT,
            // Red and green independent quads interleave as a checkerboard.
            Self::QuadList => QUAD_LIST_COLOR_DRAW_COUNT,
        }
    }

    pub const fn slot(self) -> usize {
        match self {
            Self::PointList => 0,
            Self::LineList => 1,
            Self::LineStrip => 2,
            Self::TriangleList => 3,
            Self::TriangleStrip => 4,
            Self::TriangleFan => 5,
            Self::TriangleFanMesh10 => 6,
            Self::QuadList => 7,
        }
    }

    /// Top-row number key reserved for this native topology. Key 7 selects
    /// the 100-fan mesh; Key 8 selects the seeded QUADLIST checkerboard.
    pub const fn number_key(self) -> u8 {
        match self {
            Self::QuadList => 8,
            _ => self.slot() as u8 + 1,
        }
    }

    pub const fn number_key_hid_usage(self) -> u8 {
        // USB HID usage 0x1e is the top-row `1`; `8` is 0x25.
        0x1d + self.number_key()
    }

    pub const fn vgpu_topology(self) -> u32 {
        match self {
            Self::PointList => trueos::vgpu::PRIMITIVE_TOPOLOGY_POINT_LIST,
            Self::LineList => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST,
            Self::LineStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_STRIP,
            Self::TriangleList => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            Self::TriangleStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            Self::TriangleFan => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
            Self::TriangleFanMesh10 => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
            Self::QuadList => trueos::vgpu::PRIMITIVE_TOPOLOGY_QUAD_LIST,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutionIndexCatalogue {
    pub indices: [u32; EXECUTION_INDEX_COUNT],
    pub first_indices: [u32; PRIMITIVE_MODE_COUNT],
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
        let mode_slot = mode.slot();
        let index_count = mode.indices_per_draw() as u32;
        let draw_count = mode.draw_count();
        let mut batch = IndexedDrawBatchV2 {
            clear_rgba8_srgb,
            draw_count: draw_count as u32,
            ..IndexedDrawBatchV2::default()
        };
        for draw in 0..draw_count {
            batch.draws[draw] = IndexedBatchDrawV2 {
                index_count,
                first_index: self.first_indices[mode_slot] + draw as u32 * index_count,
                base_vertex: 0,
                // The 100-fan Key 7 mesh deliberately cycles the opaque RGB
                // palette while all one-to-four draw modes retain their
                // authored per-draw palette selection.
                rgba8_srgb: colors[if mode == PrimitiveMode::TriangleFanMesh10 {
                    draw % RGB_COLOR_COUNT
                } else {
                    draw
                }],
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
        let mut cursor = 0usize;
        for (mode_slot, mode) in PrimitiveMode::ALL.into_iter().enumerate() {
            first_indices[mode_slot] = cursor as u32;
            if matches!(mode, PrimitiveMode::PointList | PrimitiveMode::LineList) {
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
                for row in 0..LINE_GRID_ROWS {
                    for step in 0..LINE_GRID_COLUMNS {
                        let column = if row.is_multiple_of(2) {
                            step
                        } else {
                            LINE_GRID_COLUMNS - 1 - step
                        };
                        indices[cursor + row * LINE_GRID_COLUMNS + step] =
                            line_grid_vertex(row * LINE_GRID_COLUMNS + column);
                    }
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
            if mode == PrimitiveMode::TriangleStrip {
                // Each row pair is woven from the same seed positions. The
                // direction and top/bottom ordering alternate so every
                // non-degenerate strip triangle remains CCW. At a row join
                // the repeated edge positions form two degenerate triangles,
                // then the next 39×1 cell band starts without a new draw.
                for row in 0..LINE_GRID_ROWS - 1 {
                    for step in 0..LINE_GRID_COLUMNS {
                        let column = if row.is_multiple_of(2) {
                            step
                        } else {
                            LINE_GRID_COLUMNS - 1 - step
                        };
                        let lower = line_grid_vertex(row * LINE_GRID_COLUMNS + column);
                        let upper = line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column);
                        let (first, second) = if row.is_multiple_of(2) {
                            (upper, lower)
                        } else {
                            (lower, upper)
                        };
                        indices[cursor..cursor + 2].copy_from_slice(&[first, second]);
                        cursor += 2;
                    }
                }
                continue;
            }
            if mode == PrimitiveMode::TriangleFan {
                // A fan has one hub. Use the seed nearest the plane centre,
                // then visit every other seed in counter-clockwise polar
                // order and repeat the first rim vertex to close the fan.
                // Collinear seeds on a shared radial line make harmless
                // degenerate wedges; every non-degenerate wedge is CCW.
                let hub = triangle_fan_hub_vertex();
                let mut rim = [0usize; LINE_GRID_VERTEX_COUNT - 1];
                let mut rim_len = 0usize;
                for vertex in 0..LINE_GRID_VERTEX_COUNT {
                    if vertex != hub {
                        rim[rim_len] = vertex;
                        rim_len += 1;
                    }
                }
                debug_assert_eq!(rim_len, rim.len());
                for candidate_index in 1..rim.len() {
                    let candidate = rim[candidate_index];
                    let mut insertion = candidate_index;
                    while insertion > 0
                        && triangle_fan_angle_precedes(candidate, rim[insertion - 1])
                    {
                        rim[insertion] = rim[insertion - 1];
                        insertion -= 1;
                    }
                    rim[insertion] = candidate;
                }
                indices[cursor] = line_grid_vertex(hub);
                cursor += 1;
                for &vertex in &rim {
                    indices[cursor] = line_grid_vertex(vertex);
                    cursor += 1;
                }
                indices[cursor] = line_grid_vertex(rim[0]);
                cursor += 1;
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
            unreachable!("every selectable primitive mode must have an execution mapping");
        }
        debug_assert_eq!(cursor, EXECUTION_INDEX_COUNT);
        ExecutionIndexCatalogue {
            indices,
            first_indices,
        }
    }
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

const fn line_grid_vertex(vertex: usize) -> u32 {
    (LINE_GRID_VERTEX_OFFSET + vertex) as u32
}

const fn triangle_fan_hub_vertex() -> usize {
    (LINE_GRID_ROWS / 2) * LINE_GRID_COLUMNS + LINE_GRID_COLUMNS / 2
}

/// Order two seed vertices counter-clockwise around the fan hub without
/// floating point. A squared-distance tie-break makes collinear ray ordering
/// deterministic; those adjacent fan wedges are degenerate by construction.
const fn triangle_fan_angle_precedes(a: usize, b: usize) -> bool {
    let hub_row = (LINE_GRID_ROWS / 2) as i32;
    let hub_column = (LINE_GRID_COLUMNS / 2) as i32;
    let ax = (a % LINE_GRID_COLUMNS) as i32 - hub_column;
    let ay = (a / LINE_GRID_COLUMNS) as i32 - hub_row;
    let bx = (b % LINE_GRID_COLUMNS) as i32 - hub_column;
    let by = (b / LINE_GRID_COLUMNS) as i32 - hub_row;
    let a_upper_half = ay > 0 || (ay == 0 && ax >= 0);
    let b_upper_half = by > 0 || (by == 0 && bx >= 0);
    if a_upper_half != b_upper_half {
        return a_upper_half;
    }
    let cross = ax * by - ay * bx;
    if cross != 0 {
        return cross > 0;
    }
    let a_radius_squared = ax * ax + ay * ay;
    let b_radius_squared = bx * bx + by * by;
    a_radius_squared < b_radius_squared
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
    fn quad_list_checkerboard_is_reserved_for_key_eight() {
        assert_eq!(PrimitiveMode::TriangleFan.number_key(), 6);
        assert_eq!(PrimitiveMode::TriangleFanMesh10.number_key(), 7);
        assert_eq!(
            PrimitiveMode::TriangleFanMesh10.number_key_hid_usage(),
            0x24
        );
        assert_eq!(PrimitiveMode::QuadList.number_key(), 8);
        assert_eq!(PrimitiveMode::QuadList.number_key_hid_usage(), 0x25);
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
    fn execution_catalogue_contains_all_eight_native_topologies() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        assert_eq!(catalogue.indices.len(), EXECUTION_INDEX_COUNT);
        let point_first = catalogue.first_indices[PrimitiveMode::PointList.slot()] as usize;
        let line_list_first = catalogue.first_indices[PrimitiveMode::LineList.slot()] as usize;
        let line_strip_first = catalogue.first_indices[PrimitiveMode::LineStrip.slot()] as usize;
        let seed: [u32; LINE_GRID_VERTEX_COUNT] = core::array::from_fn(line_grid_vertex);
        assert_eq!(
            &catalogue.indices[point_first..point_first + LINE_GRID_VERTEX_COUNT],
            &seed
        );
        assert_eq!(
            &catalogue.indices[line_list_first..line_list_first + LINE_GRID_VERTEX_COUNT],
            &seed
        );

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
                line_grid_vertex(triangle_fan_hub_vertex()),
                line_grid_vertex(triangle_fan_hub_vertex() + 1),
                line_grid_vertex(triangle_fan_hub_vertex() + 2),
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
        assert!(
            catalogue
                .indices
                .into_iter()
                .all(|index| index < EXECUTION_VERTEX_COUNT as u32)
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
    fn triangle_fan_maps_all_thousand_seed_vertices_from_a_central_hub() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        let first = catalogue.first_indices[PrimitiveMode::TriangleFan.slot()] as usize;
        let fan = &catalogue.indices[first..first + LINE_GRID_TRIANGLE_FAN_INDEX_COUNT];
        assert_eq!(fan[0], line_grid_vertex(triangle_fan_hub_vertex()));
        assert_eq!(
            fan[1],
            fan[fan.len() - 1],
            "fan rim closes at its first vertex"
        );

        let grid = line_grid_positions();
        let mut used = [false; LINE_GRID_VERTEX_COUNT];
        for &index in fan {
            used[index as usize - LINE_GRID_VERTEX_OFFSET] = true;
        }
        assert!(used.into_iter().all(|vertex| vertex));

        let hub = grid[fan[0] as usize - LINE_GRID_VERTEX_OFFSET];
        for rim in fan[1..].windows(2) {
            let b = grid[rim[0] as usize - LINE_GRID_VERTEX_OFFSET];
            let c = grid[rim[1] as usize - LINE_GRID_VERTEX_OFFSET];
            let twice_area = (b.x - hub.x) * (c.y - hub.y) - (b.y - hub.y) * (c.x - hub.x);
            assert!(
                twice_area >= -1e-6,
                "fan wedge is clockwise: area={twice_area}"
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
                assert_eq!(draw.index_count, mode.indices_per_draw() as u32);
                assert_eq!(
                    draw.first_index,
                    catalogue.first_indices[mode.slot()]
                        + draw_index as u32 * mode.indices_per_draw() as u32
                );
                assert_eq!(draw.base_vertex, 0);
                assert_eq!(
                    draw.rgba8_srgb,
                    colors[if mode == PrimitiveMode::TriangleFanMesh10 {
                        draw_index % RGB_COLOR_COUNT
                    } else {
                        draw_index
                    }]
                );
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
