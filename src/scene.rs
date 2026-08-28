//! Re-exportable authored scene document for the Potato Stamps example.
//!
//! The document itself is stored exactly in Picasso.  This module only
//! validates the returned bytes and produces the GPU-ready view after that
//! storage boundary.

use core::fmt;

pub const DOCUMENT_NAME: &str = "potato-stamps.pscene";
pub const DOCUMENT_BYTES: &[u8] = include_bytes!("../Assets/potato-stamps.pscene");
pub const STAMP_COUNT: usize = 4;
pub const VERTICES_PER_STAMP: usize = 3;
pub const VERTEX_COUNT: usize = STAMP_COUNT * VERTICES_PER_STAMP;
// Kept here rather than pulling the platform crate into the re-export format
// parser. Their values are the ABI constants checked by the binary's vGPU
// submission path: line list = 2, triangle list = 4.
const VGPU_TOPOLOGY_LINE_LIST: u32 = 2;
const VGPU_TOPOLOGY_TRIANGLE_LIST: u32 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// The immutable index catalogue has a section for every interpretation.
/// `first_index` is measured in u32 elements, matching the vGPU draw ABI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimitiveSelection {
    pub first_index: u32,
    pub index_count: u32,
    pub topology: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StampMode {
    Triangle,
    TwoLines,
    ClosedLoop,
}

impl StampMode {
    pub const fn selection(self) -> PrimitiveSelection {
        match self {
            Self::Triangle => PrimitiveSelection {
                first_index: 0,
                index_count: 3,
                topology: VGPU_TOPOLOGY_TRIANGLE_LIST,
            },
            Self::TwoLines => PrimitiveSelection {
                first_index: 3,
                index_count: 4,
                topology: VGPU_TOPOLOGY_LINE_LIST,
            },
            // TRUEOS currently has no accepted line-loop primitive. This is
            // the exact equivalent: three immutable line-list segments.
            Self::ClosedLoop => PrimitiveSelection {
                first_index: 7,
                index_count: 6,
                topology: VGPU_TOPOLOGY_LINE_LIST,
            },
        }
    }

    pub const fn from_authority_key(key: u8) -> Option<Self> {
        match key {
            0x02 => Some(Self::Triangle),
            0x03 => Some(Self::TwoLines),
            0x04 => Some(Self::ClosedLoop),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Triangle => "triangle",
            Self::TwoLines => "two-lines",
            Self::ClosedLoop => "closed-loop",
        }
    }
}

/// Expected file-format catalogue used to authenticate the parsed document.
/// Only the decoded `Scene::indices` copy is uploaded to the immutable GPU
/// buffer; every tile selects a section and changes only `base_vertex`.
pub const INDEX_CATALOGUE: [u32; 13] = [
    0, 1, 2, // CCW triangle
    0, 1, 1, 2, // two independent segments
    0, 1, 1, 2, 2, 0, // explicit loop closure
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scene {
    pub positions: [Position; VERTEX_COUNT],
    /// Parsed from the re-exportable document.  This—not `INDEX_CATALOGUE`—is
    /// the only index data allowed to reach the GPU upload path.
    pub indices: [u32; INDEX_CATALOGUE.len()],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneError {
    Header,
    VertexCount,
    Position,
    Primitive,
    TrailingData,
}

impl fmt::Display for SceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Header => "invalid Potato Stamps scene header",
            Self::VertexCount => "invalid Potato Stamps vertex count",
            Self::Position => "invalid Potato Stamps position",
            Self::Primitive => "invalid Potato Stamps primitive catalogue",
            Self::TrailingData => "unexpected Potato Stamps scene data",
        })
    }
}

impl Scene {
    pub fn decode(bytes: &[u8]) -> Result<Self, SceneError> {
        let mut tokens = Tokens::new(bytes);
        if tokens.next() != Some(b"POTATO-STAMPS/1") {
            return Err(SceneError::Header);
        }
        if parse_decimal(tokens.next()).ok_or(SceneError::VertexCount)? != VERTEX_COUNT as u32 {
            return Err(SceneError::VertexCount);
        }

        let mut positions = [Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; VERTEX_COUNT];
        for position in &mut positions {
            *position = Position {
                x: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
                y: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
                z: parse_hex_f32(tokens.next()).ok_or(SceneError::Position)?,
            };
            if !position.x.is_finite() || !position.y.is_finite() || !position.z.is_finite() {
                return Err(SceneError::Position);
            }
        }

        if tokens.next() != Some(b"TRIANGLE") {
            return Err(SceneError::Primitive);
        }
        let triangle =
            read_indices::<3>(&mut tokens, &INDEX_CATALOGUE[0..3]).ok_or(SceneError::Primitive)?;
        if tokens.next() != Some(b"TWO-LINES") {
            return Err(SceneError::Primitive);
        }
        let two_lines =
            read_indices::<4>(&mut tokens, &INDEX_CATALOGUE[3..7]).ok_or(SceneError::Primitive)?;
        if tokens.next() != Some(b"CLOSED-LOOP") {
            return Err(SceneError::Primitive);
        }
        let closed_loop =
            read_indices::<6>(&mut tokens, &INDEX_CATALOGUE[7..13]).ok_or(SceneError::Primitive)?;
        if tokens.next().is_some() {
            return Err(SceneError::TrailingData);
        }
        if !all_triangles_are_ccw(&positions) {
            return Err(SceneError::Primitive);
        }
        let mut indices = [0; INDEX_CATALOGUE.len()];
        indices[0..3].copy_from_slice(&triangle);
        indices[3..7].copy_from_slice(&two_lines);
        indices[7..13].copy_from_slice(&closed_loop);
        Ok(Self { positions, indices })
    }

    pub fn position_bytes(&self) -> &[u8] {
        // Position is repr(C), three f32s, and is the authenticated
        // Float32x3 layout consumed by the immediate Picasso shader package.
        unsafe {
            core::slice::from_raw_parts(
                self.positions.as_ptr().cast::<u8>(),
                core::mem::size_of_val(&self.positions),
            )
        }
    }

    pub fn index_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.indices.as_ptr().cast::<u8>(),
                core::mem::size_of_val(&self.indices),
            )
        }
    }
}

fn all_triangles_are_ccw(positions: &[Position; VERTEX_COUNT]) -> bool {
    positions.chunks_exact(VERTICES_PER_STAMP).all(|triangle| {
        let a = triangle[0];
        let b = triangle[1];
        let c = triangle[2];
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) > 0.0
    })
}

fn read_indices<const N: usize>(tokens: &mut Tokens<'_>, expected: &[u32]) -> Option<[u32; N]> {
    if expected.len() != N {
        return None;
    }
    let mut actual = [0; N];
    for index in &mut actual {
        *index = parse_decimal(tokens.next())?;
    }
    (actual.as_slice() == expected).then_some(actual)
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
    fn authored_scene_is_valid_and_ccw() {
        let scene = Scene::decode(DOCUMENT_BYTES).unwrap();
        assert_eq!(scene.positions.len(), VERTEX_COUNT);
        assert_eq!(scene.indices, INDEX_CATALOGUE);
        assert!(all_triangles_are_ccw(&scene.positions));
    }

    #[test]
    fn primitive_catalogue_never_requires_a_buffer_rewrite() {
        assert_eq!(StampMode::Triangle.selection().first_index, 0);
        assert_eq!(StampMode::TwoLines.selection().first_index, 3);
        assert_eq!(StampMode::ClosedLoop.selection().first_index, 7);
        assert_eq!(INDEX_CATALOGUE.len(), 13);
    }
}
