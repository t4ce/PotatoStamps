//! Adjacency-topology geometry skeleton.
//!
//! The native `_ADJ` modes are intentionally kept separate from the ordinary
//! grid catalogue.  They currently submit one minimum-size, fully degenerate
//! primitive: this preserves the topology selection and makes the unresolved
//! geometry-shader semantics visible without inventing rendered geometry.
//!
//! This module owns only adjacency facts and catalogue emission.  It does not
//! add a geometry-shader implementation.

use super::{PrimitiveMode, base::line_grid_vertex};

/// Minimum legal input sizes used by the current clear-only placeholders.
pub(super) const LINE_LIST_INDEX_COUNT: usize = 4;
pub(super) const LINE_STRIP_INDEX_COUNT: usize = 4;
pub(super) const TRIANGLE_LIST_INDEX_COUNT: usize = 6;
pub(super) const TRIANGLE_STRIP_INDEX_COUNT: usize = 6;

/// The adjacency topologies exposed by the number-key toggles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AdjacencyPrimitive {
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl AdjacencyPrimitive {
    pub(super) const fn from_mode(mode: PrimitiveMode) -> Option<Self> {
        match mode {
            PrimitiveMode::LineListAdj => Some(Self::LineList),
            PrimitiveMode::LineStripAdj => Some(Self::LineStrip),
            PrimitiveMode::TriangleListAdj => Some(Self::TriangleList),
            PrimitiveMode::TriangleStripAdj => Some(Self::TriangleStrip),
            _ => None,
        }
    }

    pub(super) const fn index_count(self) -> usize {
        match self {
            Self::LineList => LINE_LIST_INDEX_COUNT,
            Self::LineStrip => LINE_STRIP_INDEX_COUNT,
            Self::TriangleList => TRIANGLE_LIST_INDEX_COUNT,
            Self::TriangleStrip => TRIANGLE_STRIP_INDEX_COUNT,
        }
    }
}

/// True only for the four native adjacency interpretations.
pub(super) const fn is_adjacency_mode(mode: PrimitiveMode) -> bool {
    AdjacencyPrimitive::from_mode(mode).is_some()
}

/// Return the placeholder input count for an adjacency topology.
pub(super) const fn adjacency_index_count(mode: PrimitiveMode) -> usize {
    match AdjacencyPrimitive::from_mode(mode) {
        Some(primitive) => primitive.index_count(),
        None => 0,
    }
}

/// Append one degenerate adjacency catalogue entry and return its count.
///
/// Every input is the first seeded grid vertex.  The caller still binds the
/// actual native adjacency topology, but no geometry-shader interpretation is
/// assumed here and therefore no visible geometry is emitted.
pub(super) fn append_degenerate_indices(
    indices: &mut [u32],
    cursor: &mut usize,
    mode: PrimitiveMode,
) -> usize {
    let count = adjacency_index_count(mode);
    debug_assert!(is_adjacency_mode(mode));
    debug_assert!(*cursor + count <= indices.len());
    let end = *cursor + count;
    for index in &mut indices[*cursor..end] {
        *index = line_grid_vertex(0);
    }
    *cursor = end;
    count
}
