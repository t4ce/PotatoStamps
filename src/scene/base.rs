//! Ordinary (non-adjacency) geometry construction helpers.
//!
//! This child module intentionally contains only the geometry mappings for the
//! regular grid, the triangle-fan partitions, the independent RECTLIST
//! records, and the QUADSTRIP ring.  The scene catalogue and native topology
//! selection remain owned by the parent module; these helpers only translate
//! logical geometry into positions or execution-buffer indices.

use super::*;

/// Two independent closed native QUADSTRIP annuli.
pub const QUAD_STRIP_RING_COUNT: usize = 2;
pub const QUAD_STRIP_RING_SEGMENTS: usize = 64;
pub const QUAD_STRIP_RING_VERTICES_PER_PAIR: usize = 2;
pub const QUAD_STRIP_RING_RADII: [[f32; 2]; QUAD_STRIP_RING_COUNT] =
    [[0.77, 0.86], [0.58, 0.67]];
pub const QUAD_STRIP_RING_VERTICES_PER_RING: usize =
    QUAD_STRIP_RING_SEGMENTS * QUAD_STRIP_RING_VERTICES_PER_PAIR;
pub const QUAD_STRIP_RING_VERTEX_OFFSET: usize = VERTEX_COUNT;
pub const QUAD_STRIP_RING_VERTEX_COUNT: usize =
    QUAD_STRIP_RING_COUNT * QUAD_STRIP_RING_VERTICES_PER_RING;
pub const QUAD_STRIP_DRAW_COUNT: usize = QUAD_STRIP_RING_COUNT;
pub const QUAD_STRIP_INDICES_PER_DRAW: usize =
    (QUAD_STRIP_RING_SEGMENTS + 1) * QUAD_STRIP_RING_VERTICES_PER_PAIR;

/// The two annuli expose four independent 64-vertex circular paths.
pub const RING_CIRCLE_COUNT: usize =
    QUAD_STRIP_RING_COUNT * QUAD_STRIP_RING_VERTICES_PER_PAIR;
pub const RING_CIRCLE_VERTEX_COUNT: usize = QUAD_STRIP_RING_SEGMENTS;
pub const POINT_RING_DRAW_COUNT: usize = RING_CIRCLE_COUNT;
pub const POINT_RING_INDICES_PER_DRAW: usize = RING_CIRCLE_VERTEX_COUNT;
pub const POINT_RING_INDEX_COUNT: usize = POINT_RING_DRAW_COUNT * POINT_RING_INDICES_PER_DRAW;
pub const LINE_LIST_RING_DRAW_COUNT: usize = RING_CIRCLE_COUNT;
pub const LINE_LIST_RING_INDICES_PER_DRAW: usize = RING_CIRCLE_VERTEX_COUNT;
pub const LINE_LIST_RING_INDEX_COUNT: usize =
    LINE_LIST_RING_DRAW_COUNT * LINE_LIST_RING_INDICES_PER_DRAW;
pub const LINE_STRIP_RING_DRAW_COUNT: usize = RING_CIRCLE_COUNT;
pub const LINE_STRIP_RING_INDICES_PER_DRAW: usize = RING_CIRCLE_VERTEX_COUNT + 1;
pub const LINE_STRIP_RING_INDEX_COUNT: usize =
    LINE_STRIP_RING_DRAW_COUNT * LINE_STRIP_RING_INDICES_PER_DRAW;

/// Shared 32×32 seed plane for the regular point, line, and triangle modes.
pub const LINE_GRID_COLUMNS: usize = 32;
pub const LINE_GRID_ROWS: usize = 32;
pub const LINE_GRID_VERTEX_COUNT: usize = LINE_GRID_COLUMNS * LINE_GRID_ROWS;
pub const LINE_GRID_VERTEX_OFFSET: usize =
    QUAD_STRIP_RING_VERTEX_OFFSET + QUAD_STRIP_RING_VERTEX_COUNT;
pub const LINE_GRID_XYZ_BYTES: usize = LINE_GRID_VERTEX_COUNT * 12;
pub const RGB_COLOR_COUNT: usize = 3;

pub const POINT_GRID_GREEN_INDEX_COUNT: usize = (LINE_GRID_VERTEX_COUNT + 2) / 3;
pub const POINT_GRID_RED_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT - POINT_GRID_GREEN_INDEX_COUNT;

pub const LINE_GRID_TRIANGLE_LIST_INDEX_COUNT: usize =
    (LINE_GRID_COLUMNS - 1) * (LINE_GRID_ROWS - 1) * 6;
pub const LINE_GRID_TRIANGLE_COUNT: usize = LINE_GRID_TRIANGLE_LIST_INDEX_COUNT / 3;
pub const TRIANGLE_LIST_COLOR0_INDEX_COUNT: usize = LINE_GRID_TRIANGLE_COUNT.div_ceil(3) * 3;
pub const TRIANGLE_LIST_COLOR1_INDEX_COUNT: usize = (LINE_GRID_TRIANGLE_COUNT + 1) / 3 * 3;
pub const TRIANGLE_LIST_COLOR2_INDEX_COUNT: usize = LINE_GRID_TRIANGLE_COUNT / 3 * 3;

pub const LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT: usize =
    (LINE_GRID_ROWS - 1) * LINE_GRID_COLUMNS * 2;
pub const LINE_GRID_TRIANGLE_FAN_INDEX_COUNT: usize = LINE_GRID_VERTEX_COUNT;
pub const TRIANGLE_FAN_VERTEX_COUNTS: [usize; 7] = [1024, 512, 256, 64, 32, 16, 8];
pub const TRIANGLE_FAN_CATALOGUE_INDEX_COUNT: usize =
    LINE_GRID_VERTEX_COUNT * TRIANGLE_FAN_VERTEX_COUNTS.len();

/// Original Key 8 mode: non-overlapping 2×2 blocks from the 32×32 seed grid.
pub const QUAD_LIST_COLUMNS: usize = LINE_GRID_COLUMNS / 2;
pub const QUAD_LIST_ROWS: usize = LINE_GRID_ROWS / 2;
pub const QUAD_LIST_CELL_COUNT: usize = QUAD_LIST_COLUMNS * QUAD_LIST_ROWS;
pub const QUAD_LIST_COLOR_DRAW_COUNT: usize = 2;
pub const QUAD_LIST_COLOR0_CELL_COUNT: usize = QUAD_LIST_CELL_COUNT.div_ceil(2);
pub const QUAD_LIST_COLOR1_CELL_COUNT: usize = QUAD_LIST_CELL_COUNT / 2;
pub const QUAD_LIST_COLOR0_INDEX_COUNT: usize = QUAD_LIST_COLOR0_CELL_COUNT * 4;
pub const QUAD_LIST_COLOR1_INDEX_COUNT: usize = QUAD_LIST_COLOR1_CELL_COUNT * 4;
pub const QUAD_LIST_INDEX_COUNT: usize = QUAD_LIST_CELL_COUNT * 4;

/// Alternate Key 8 mode: the Key 9 ring vertices grouped without strip reuse.
pub const QUAD_LIST_RING_QUADS_PER_RING: usize = QUAD_STRIP_RING_SEGMENTS / 2;
pub const QUAD_LIST_RING_CELL_COUNT: usize =
    QUAD_STRIP_RING_COUNT * QUAD_LIST_RING_QUADS_PER_RING;
pub const QUAD_LIST_RING_DRAW_COUNT: usize = QUAD_STRIP_RING_COUNT;
pub const QUAD_LIST_RING_INDICES_PER_DRAW: usize = QUAD_STRIP_RING_VERTICES_PER_RING;
pub const QUAD_LIST_RING_INDEX_COUNT: usize = QUAD_STRIP_RING_VERTEX_COUNT;

pub const RECT_LIST_SCREEN_WIDTH_PX: f32 = 640.0;
pub const RECT_LIST_SCREEN_HEIGHT_PX: f32 = 360.0;
pub const RECT_LIST_VERTEX_OFFSET: usize = LINE_GRID_VERTEX_OFFSET + LINE_GRID_VERTEX_COUNT;
pub const RECT_LIST_VERTICES_PER_RECTANGLE: usize = 3;
pub const RECT_LIST_RECT_WIDTH_PX: f32 = 8.0;
pub const RECT_LIST_RECT_HEIGHT_PX: f32 = 8.0;
pub const RECT_LIST_CELL_COUNT: usize = QUAD_STRIP_RING_COUNT * QUAD_STRIP_RING_SEGMENTS;
pub const RECT_LIST_VERTEX_COUNT: usize = RECT_LIST_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const RECT_LIST_COLOR_DRAW_COUNT: usize = QUAD_STRIP_RING_COUNT;
pub const RECT_LIST_COLOR0_CELL_COUNT: usize = QUAD_STRIP_RING_SEGMENTS;
pub const RECT_LIST_COLOR1_CELL_COUNT: usize = QUAD_STRIP_RING_SEGMENTS;
pub const RECT_LIST_COLOR0_INDEX_COUNT: usize =
    RECT_LIST_COLOR0_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const RECT_LIST_COLOR1_INDEX_COUNT: usize =
    RECT_LIST_COLOR1_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const RECT_LIST_INDEX_COUNT: usize = RECT_LIST_CELL_COUNT * RECT_LIST_VERTICES_PER_RECTANGLE;
pub const EXECUTION_VERTEX_COUNT: usize = RECT_LIST_VERTEX_OFFSET + RECT_LIST_VERTEX_COUNT;

/// Alternating inner/outer ring positions used by Key 9's QUADSTRIP.
pub fn quad_strip_ring_positions() -> [Position; QUAD_STRIP_RING_VERTEX_COUNT] {
    core::array::from_fn(|vertex| {
        let ring = vertex / QUAD_STRIP_RING_VERTICES_PER_RING;
        let ring_vertex = vertex % QUAD_STRIP_RING_VERTICES_PER_RING;
        let pair = ring_vertex / QUAD_STRIP_RING_VERTICES_PER_PAIR;
        let outer = ring_vertex % QUAD_STRIP_RING_VERTICES_PER_PAIR == 1;
        let angle = core::f32::consts::TAU * pair as f32 / QUAD_STRIP_RING_SEGMENTS as f32;
        // Each band is half the original 0.18 radius, with a visible gap
        // between the independently submitted outer and inner strips.
        let radius = QUAD_STRIP_RING_RADII[ring][if outer { 1 } else { 0 }];
        Position {
            // Keep the ring circular in the demo's 16:9 presentation.
            x: trueos_math::cos_f32(angle)
                * radius
                * (RECT_LIST_SCREEN_HEIGHT_PX / RECT_LIST_SCREEN_WIDTH_PX),
            y: trueos_math::sin_f32(angle) * radius,
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
        let ring = rectangle / QUAD_STRIP_RING_SEGMENTS;
        let pair = rectangle % QUAD_STRIP_RING_SEGMENTS;
        let radius = (QUAD_STRIP_RING_RADII[ring][0] + QUAD_STRIP_RING_RADII[ring][1]) * 0.5;
        let angle = core::f32::consts::TAU * pair as f32 / QUAD_STRIP_RING_SEGMENTS as f32;
        let ndc_x = trueos_math::cos_f32(angle)
            * radius
            * (RECT_LIST_SCREEN_HEIGHT_PX / RECT_LIST_SCREEN_WIDTH_PX);
        let ndc_y = trueos_math::sin_f32(angle) * radius;
        let center_x =
            0.5 + (ndc_x + 1.0) * 0.5 * (RECT_LIST_SCREEN_WIDTH_PX - 1.0);
        let center_y =
            0.5 + (1.0 - ndc_y) * 0.5 * (RECT_LIST_SCREEN_HEIGHT_PX - 1.0);
        let left = center_x - RECT_LIST_RECT_WIDTH_PX * 0.5;
        let right = center_x + RECT_LIST_RECT_WIDTH_PX * 0.5;
        let upper = center_y - RECT_LIST_RECT_HEIGHT_PX * 0.5;
        let lower = center_y + RECT_LIST_RECT_HEIGHT_PX * 0.5;
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

pub(super) const fn quad_strip_ring_vertex(ring: usize, pair: usize, radial: usize) -> u32 {
    (QUAD_STRIP_RING_VERTEX_OFFSET
        + ring * QUAD_STRIP_RING_VERTICES_PER_RING
        + pair * QUAD_STRIP_RING_VERTICES_PER_PAIR
        + radial) as u32
}

pub(super) const fn ring_circle_vertex(circle: usize, step: usize) -> u32 {
    quad_strip_ring_vertex(
        circle / QUAD_STRIP_RING_VERTICES_PER_PAIR,
        step % QUAD_STRIP_RING_SEGMENTS,
        circle % QUAD_STRIP_RING_VERTICES_PER_PAIR,
    )
}

pub(super) const fn line_grid_vertex(vertex: usize) -> u32 {
    (LINE_GRID_VERTEX_OFFSET + vertex) as u32
}

/// Snake order for the ordinary LINESTRIP across the 32×32 seed grid.
pub(super) const fn line_grid_snake_seed_vertex(step: usize) -> usize {
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
/// in each row band.
pub(super) const fn triangle_strip_main_seed_vertex(step: usize) -> usize {
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

pub(super) const fn triangle_strip_main_vertex(step: usize) -> u32 {
    line_grid_vertex(triangle_strip_main_seed_vertex(step))
}

pub(super) const fn rect_list_vertex(rectangle: usize, corner: usize) -> u32 {
    (RECT_LIST_VERTEX_OFFSET + rectangle * RECT_LIST_VERTICES_PER_RECTANGLE + corner) as u32
}

/// Map one fan-local slot into its rectangular spatial partition. Every stage
/// consumes all 1,024 seeds exactly once.
pub(super) fn triangle_fan_partition_seed_vertex(
    fan_size: usize,
    fan: usize,
    slot: usize,
) -> usize {
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

/// Append the immutable index range for one ordinary primitive mode.
/// Adjacency modes deliberately live in `adjacency.rs` and return `false`.
pub(super) fn append_indices(
    mode: PrimitiveMode,
    indices: &mut [u32],
    cursor: &mut usize,
    fan_first_indices: &mut [u32; TRIANGLE_FAN_VERTEX_COUNTS.len()],
    fan_index_counts: &mut [u32; TRIANGLE_FAN_VERTEX_COUNTS.len()],
) -> bool {
    match mode {
        PrimitiveMode::PointList => append_point_list(indices, cursor),
        PrimitiveMode::PointListRings => append_point_list_rings(indices, cursor),
        PrimitiveMode::LineList => append_line_list(indices, cursor),
        PrimitiveMode::LineListRings => append_line_list_rings(indices, cursor),
        PrimitiveMode::LineStrip => append_line_strip(indices, cursor),
        PrimitiveMode::LineStripRings => append_line_strip_rings(indices, cursor),
        PrimitiveMode::TriangleList => append_triangle_list(indices, cursor),
        PrimitiveMode::TriangleStrip => append_triangle_strip(indices, cursor),
        PrimitiveMode::TriangleFan => {
            append_triangle_fans(indices, cursor, fan_first_indices, fan_index_counts)
        }
        PrimitiveMode::QuadList => append_quad_list(indices, cursor),
        PrimitiveMode::QuadListRings => append_quad_list_rings(indices, cursor),
        PrimitiveMode::QuadStrip => append_quad_strip(indices, cursor),
        PrimitiveMode::RectList => append_rect_list(indices, cursor),
        PrimitiveMode::LineListAdj
        | PrimitiveMode::LineStripAdj
        | PrimitiveMode::TriangleListAdj
        | PrimitiveMode::TriangleStripAdj => return false,
    }
    true
}

fn append_point_list(indices: &mut [u32], cursor: &mut usize) {
    let first = *cursor;
    // Store each immediate-color partition contiguously. The source ordinal,
    // rather than screen coordinate, controls green, red, red repetition.
    for grid_vertex in 0..LINE_GRID_VERTEX_COUNT {
        if grid_vertex % RGB_COLOR_COUNT == 0 {
            indices[*cursor] = line_grid_vertex(grid_vertex);
            *cursor += 1;
        }
    }
    debug_assert_eq!(*cursor, first + POINT_GRID_GREEN_INDEX_COUNT);
    for grid_vertex in 0..LINE_GRID_VERTEX_COUNT {
        if grid_vertex % RGB_COLOR_COUNT != 0 {
            indices[*cursor] = line_grid_vertex(grid_vertex);
            *cursor += 1;
        }
    }
}

fn append_point_list_rings(indices: &mut [u32], cursor: &mut usize) {
    for circle in 0..RING_CIRCLE_COUNT {
        for step in 0..RING_CIRCLE_VERTEX_COUNT {
            indices[*cursor] = ring_circle_vertex(circle, step);
            *cursor += 1;
        }
    }
}

fn append_line_list(indices: &mut [u32], cursor: &mut usize) {
    for grid_vertex in 0..LINE_GRID_VERTEX_COUNT {
        indices[*cursor + grid_vertex] = line_grid_vertex(grid_vertex);
    }
    *cursor += LINE_GRID_VERTEX_COUNT;
}

fn append_line_list_rings(indices: &mut [u32], cursor: &mut usize) {
    // Sequential disjoint pairs produce 32 dashes per circle. No vertex is
    // reused to bridge one dash to the next.
    for circle in 0..RING_CIRCLE_COUNT {
        for step in 0..RING_CIRCLE_VERTEX_COUNT {
            indices[*cursor] = ring_circle_vertex(circle, step);
            *cursor += 1;
        }
    }
}

fn append_line_strip(indices: &mut [u32], cursor: &mut usize) {
    // Visit each row opposite its neighbor, using every seed exactly once.
    for step in 0..LINE_GRID_VERTEX_COUNT {
        indices[*cursor + step] = line_grid_vertex(line_grid_snake_seed_vertex(step));
    }
    *cursor += LINE_GRID_VERTEX_COUNT;
}

fn append_line_strip_rings(indices: &mut [u32], cursor: &mut usize) {
    // Each circle is a separate strip draw; repeat only its first point to
    // close the final segment without connecting distinct radii.
    for circle in 0..RING_CIRCLE_COUNT {
        for step in 0..=RING_CIRCLE_VERTEX_COUNT {
            indices[*cursor] = ring_circle_vertex(circle, step);
            *cursor += 1;
        }
    }
}

fn append_triangle_list(indices: &mut [u32], cursor: &mut usize) {
    // Two CCW triangles per cell, packed into red, green, and blue draws.
    let first = *cursor;
    for color in 0..RGB_COLOR_COUNT {
        let (color_offset, color_count) = match color {
            0 => (0, TRIANGLE_LIST_COLOR0_INDEX_COUNT),
            1 => (
                TRIANGLE_LIST_COLOR0_INDEX_COUNT,
                TRIANGLE_LIST_COLOR1_INDEX_COUNT,
            ),
            _ => (
                TRIANGLE_LIST_COLOR0_INDEX_COUNT + TRIANGLE_LIST_COLOR1_INDEX_COUNT,
                TRIANGLE_LIST_COLOR2_INDEX_COUNT,
            ),
        };
        let mut color_cursor = first + color_offset;
        for row in 0..LINE_GRID_ROWS - 1 {
            for column in 0..LINE_GRID_COLUMNS - 1 {
                let lower_left = line_grid_vertex(row * LINE_GRID_COLUMNS + column);
                let lower_right = line_grid_vertex(row * LINE_GRID_COLUMNS + column + 1);
                let upper_left = line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column);
                let upper_right = line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column + 1);
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
        debug_assert_eq!(color_cursor, first + color_offset + color_count);
    }
    *cursor += LINE_GRID_TRIANGLE_LIST_INDEX_COUNT;
}

fn append_triangle_strip(indices: &mut [u32], cursor: &mut usize) {
    for step in 0..LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT {
        indices[*cursor + step] = triangle_strip_main_vertex(step);
    }
    *cursor += LINE_GRID_TRIANGLE_STRIP_INDEX_COUNT;
}

fn append_triangle_fans(
    indices: &mut [u32],
    cursor: &mut usize,
    fan_first_indices: &mut [u32; TRIANGLE_FAN_VERTEX_COUNTS.len()],
    fan_index_counts: &mut [u32; TRIANGLE_FAN_VERTEX_COUNTS.len()],
) {
    // Tile layouts: 32×32, 16×32, 16×16, 8×8, 8×4, 4×4, and 4×2.
    for (fan_index, &fan_size) in TRIANGLE_FAN_VERTEX_COUNTS.iter().enumerate() {
        fan_first_indices[fan_index] = *cursor as u32;
        fan_index_counts[fan_index] = fan_size as u32;
        for fan in 0..LINE_GRID_VERTEX_COUNT / fan_size {
            for slot in 0..fan_size {
                indices[*cursor] = line_grid_vertex(triangle_fan_partition_seed_vertex(
                    fan_size, fan, slot,
                ));
                *cursor += 1;
            }
        }
    }
}

fn append_quad_list(indices: &mut [u32], cursor: &mut usize) {
    // Original Key 8 interpretation: each independent quad consumes one
    // non-overlapping 2×2 block from the 32×32 seed grid.
    let first = *cursor;
    for color in 0..QUAD_LIST_COLOR_DRAW_COUNT {
        let color_first = if color == 0 {
            first
        } else {
            first + QUAD_LIST_COLOR0_INDEX_COUNT
        };
        let mut color_cursor = color_first;
        for quad_row in 0..QUAD_LIST_ROWS {
            for quad_column in 0..QUAD_LIST_COLUMNS {
                if (quad_row + quad_column) % QUAD_LIST_COLOR_DRAW_COUNT != color {
                    continue;
                }
                let row = quad_row * 2;
                let column = quad_column * 2;
                indices[color_cursor..color_cursor + 4].copy_from_slice(&[
                    line_grid_vertex(row * LINE_GRID_COLUMNS + column),
                    line_grid_vertex(row * LINE_GRID_COLUMNS + column + 1),
                    line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column + 1),
                    line_grid_vertex((row + 1) * LINE_GRID_COLUMNS + column),
                ]);
                color_cursor += 4;
            }
        }
        let color_count = if color == 0 {
            QUAD_LIST_COLOR0_INDEX_COUNT
        } else {
            QUAD_LIST_COLOR1_INDEX_COUNT
        };
        debug_assert_eq!(color_cursor, color_first + color_count);
    }
    *cursor += QUAD_LIST_INDEX_COUNT;
}

fn append_quad_list_rings(indices: &mut [u32], cursor: &mut usize) {
    // Reinterpret the exact Key 9 vertex stream as independent groups of four.
    // Each group spans two radial pairs, so every following strip segment is
    // absent and becomes a visible angular gap.
    for ring in 0..QUAD_STRIP_RING_COUNT {
        for pair in (0..QUAD_STRIP_RING_SEGMENTS).step_by(2) {
            indices[*cursor..*cursor + 4].copy_from_slice(&[
                quad_strip_ring_vertex(ring, pair, 0),
                quad_strip_ring_vertex(ring, pair, 1),
                quad_strip_ring_vertex(ring, pair + 1, 1),
                quad_strip_ring_vertex(ring, pair + 1, 0),
            ]);
            *cursor += 4;
        }
    }
}

fn append_quad_strip(indices: &mut [u32], cursor: &mut usize) {
    // Each ring is its own draw. Inner/outer pairs progress counter-clockwise;
    // pair zero repeats only at the end of that ring to close it.
    for ring in 0..QUAD_STRIP_RING_COUNT {
        for pair in 0..=QUAD_STRIP_RING_SEGMENTS {
            let ring_pair = pair % QUAD_STRIP_RING_SEGMENTS;
            indices[*cursor] = quad_strip_ring_vertex(ring, ring_pair, 0);
            indices[*cursor + 1] = quad_strip_ring_vertex(ring, ring_pair, 1);
            *cursor += 2;
        }
    }
}

fn append_rect_list(indices: &mut [u32], cursor: &mut usize) {
    for ring in 0..QUAD_STRIP_RING_COUNT {
        for pair in 0..QUAD_STRIP_RING_SEGMENTS {
            let rectangle = ring * QUAD_STRIP_RING_SEGMENTS + pair;
            indices[*cursor..*cursor + RECT_LIST_VERTICES_PER_RECTANGLE].copy_from_slice(&[
                rect_list_vertex(rectangle, 0),
                rect_list_vertex(rectangle, 1),
                rect_list_vertex(rectangle, 2),
            ]);
            *cursor += RECT_LIST_VERTICES_PER_RECTANGLE;
        }
    }
}
