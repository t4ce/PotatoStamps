//! Re-exportable authored scene document for the Potato Stamps example.
//!
//! The document is stored exactly in Picasso. This module only validates the
//! bytes returned across that storage boundary and produces the GPU-ready view
//! afterwards.

use core::fmt;

pub const DOCUMENT_NAME: &str = "potato-stamps.pscene";
pub const DOCUMENT_BYTES: &[u8] = include_bytes!("../Assets/potato-stamps.pscene");
pub const COLOR_TEXTURE_NAME: &str = "potato-stamps-colors.bmp";
pub const STAMP_COUNT: usize = 4;
pub const VERTICES_PER_STAMP: usize = 3;
pub const VERTEX_COUNT: usize = STAMP_COUNT * VERTICES_PER_STAMP;
pub const INDEX_COUNT: usize = VERTEX_COUNT;

/// A four-texel opaque BMP: red, green, blue, and yellow. The bytes seed
/// Picasso, then must be read back from Picasso before media decoding.
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
    /// The retained textured carrier currently samples its interpolated
    /// normal XY. These authored values select one solid texel per triangle.
    pub texture_u: f32,
    pub texture_v: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scene {
    pub positions: [Position; VERTEX_COUNT],
    pub indices: [u32; INDEX_COUNT],
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
    }
}
