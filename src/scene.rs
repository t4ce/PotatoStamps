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
pub const STAMP_COUNT: usize = 4;
pub const VERTICES_PER_STAMP: usize = 3;
pub const VERTEX_COUNT: usize = STAMP_COUNT * VERTICES_PER_STAMP;
pub const INDEX_COUNT: usize = VERTEX_COUNT;
pub const PRIMITIVE_MODE_COUNT: usize = 6;
pub const EXECUTION_INDEX_COUNT: usize = STAMP_COUNT * (3 + 6 + 4 + 3 + 3 + 3);

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
}

impl PrimitiveMode {
    pub const ALL: [Self; PRIMITIVE_MODE_COUNT] = [
        Self::PointList,
        Self::LineList,
        Self::LineStrip,
        Self::TriangleList,
        Self::TriangleStrip,
        Self::TriangleFan,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PointList => "point-list",
            Self::LineList => "line-list-closed",
            Self::LineStrip => "line-strip-closed",
            Self::TriangleList => "triangle-list",
            Self::TriangleStrip => "triangle-strip",
            Self::TriangleFan => "triangle-fan",
        }
    }

    pub const fn indices_per_stamp(self) -> usize {
        match self {
            Self::PointList | Self::TriangleList | Self::TriangleStrip | Self::TriangleFan => 3,
            Self::LineList => 6,
            Self::LineStrip => 4,
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
        }
    }

    pub const fn vgpu_topology(self) -> u32 {
        match self {
            Self::PointList => trueos::vgpu::PRIMITIVE_TOPOLOGY_POINT_LIST,
            Self::LineList => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_LIST,
            Self::LineStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_LINE_STRIP,
            Self::TriangleList => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            Self::TriangleStrip => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            Self::TriangleFan => trueos::vgpu::PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
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
        let index_count = mode.indices_per_stamp() as u32;
        let mut batch = IndexedDrawBatchV2 {
            clear_rgba8_srgb,
            draw_count: STAMP_COUNT as u32,
            ..IndexedDrawBatchV2::default()
        };
        for (stamp, color) in colors.into_iter().enumerate() {
            batch.draws[stamp] = IndexedBatchDrawV2 {
                index_count,
                first_index: self.first_indices[mode_slot] + stamp as u32 * index_count,
                base_vertex: 0,
                rgba8_srgb: color,
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

    /// Build every native primitive interpretation once from the immutable
    /// triangle corners read back from Picasso. Frame submission only selects
    /// ranges from this catalogue; it never rewrites execution-buffer bytes.
    pub fn execution_index_catalogue(&self) -> ExecutionIndexCatalogue {
        let mut indices = [0; EXECUTION_INDEX_COUNT];
        let mut first_indices = [0; PRIMITIVE_MODE_COUNT];
        let mut cursor = 0usize;
        for (mode_slot, mode) in PrimitiveMode::ALL.into_iter().enumerate() {
            first_indices[mode_slot] = cursor as u32;
            for stamp in 0..STAMP_COUNT {
                let start = stamp * VERTICES_PER_STAMP;
                let [a, b, c] = self.indices[start..start + VERTICES_PER_STAMP] else {
                    unreachable!()
                };
                let sequence: &[u32] = match mode {
                    PrimitiveMode::PointList
                    | PrimitiveMode::TriangleList
                    | PrimitiveMode::TriangleStrip
                    | PrimitiveMode::TriangleFan => &[a, b, c],
                    PrimitiveMode::LineList => &[a, b, b, c, c, a],
                    PrimitiveMode::LineStrip => &[a, b, c, a],
                };
                indices[cursor..cursor + sequence.len()].copy_from_slice(sequence);
                cursor += sequence.len();
            }
        }
        debug_assert_eq!(cursor, EXECUTION_INDEX_COUNT);
        ExecutionIndexCatalogue {
            indices,
            first_indices,
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
    fn execution_catalogue_contains_all_six_native_topologies() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        let catalogue = scene.execution_index_catalogue();
        assert_eq!(catalogue.indices.len(), EXECUTION_INDEX_COUNT);
        for (slot, mode) in PrimitiveMode::ALL.into_iter().enumerate() {
            let first = catalogue.first_indices[slot] as usize;
            let count = mode.indices_per_stamp();
            let expected: &[u32] = match mode {
                PrimitiveMode::PointList
                | PrimitiveMode::TriangleList
                | PrimitiveMode::TriangleStrip
                | PrimitiveMode::TriangleFan => &[0, 1, 2],
                PrimitiveMode::LineList => &[0, 1, 1, 2, 2, 0],
                PrimitiveMode::LineStrip => &[0, 1, 2, 0],
            };
            assert_eq!(&catalogue.indices[first..first + count], expected,);
        }
        assert!(
            catalogue
                .indices
                .into_iter()
                .all(|index| index < VERTEX_COUNT as u32)
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
            assert_eq!(batch.draw_count, STAMP_COUNT as u32);
            for (stamp, draw) in batch.draws[..STAMP_COUNT].iter().enumerate() {
                assert_eq!(draw.topology, mode.vgpu_topology());
                assert_eq!(draw.index_count, mode.indices_per_stamp() as u32);
                assert_eq!(
                    draw.first_index,
                    catalogue.first_indices[mode.slot()]
                        + stamp as u32 * mode.indices_per_stamp() as u32
                );
                assert_eq!(draw.base_vertex, 0);
                assert_eq!(draw.rgba8_srgb, colors[stamp]);
                assert_eq!(draw.reserved, 0);
            }
            for draw in &batch.draws[STAMP_COUNT..] {
                assert_eq!(draw.index_count, 0);
                assert_eq!(draw.topology, 0);
                assert_eq!(draw.reserved, 0);
            }
        }
    }
}
