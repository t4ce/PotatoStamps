# Potato Stamps

`Potato Stamps` is a deliberately small, standalone Picasso/TRUEOS example.
It is package-ready (including its native TRUEOS target configuration) but has
no Blueprint registration or autostart entry.

## Asset path

`Assets/potato-stamps.pscene` is the authored, re-exportable scene document.
At boot it is inserted exactly into the example's private in-memory Picasso
redb. The runtime reader obtains a fresh copy from Picasso, validates and
decodes it, then and only then uploads the decoded positions and immutable
index catalogue to vGPU buffers.

```text
potato-stamps.pscene -> Picasso private in-memory redb -> decoder -> vGPU buffers
```

There is no build-time prepared render buffer and no second asset path.

## Primitive experiment

The document holds four tiled copies of the same intentionally CCW three-point
stamp.  Its immutable index catalogue has three interpretations of those same
three positions:

| Mode | Topology | Index sequence |
| --- | --- | --- |
| Triangle | triangle list | `0, 1, 2` |
| Two lines | line list | `0, 1, 1, 2` |
| Closed loop | line list | `0, 1, 1, 2, 2, 0` |

The vertex and index buffers are written once, during initialization. Press
`1`, `2`, or `3` to change the CPU authority signal; the frame submission then
selects a pre-seeded range/topology without rebuilding or re-uploading either
execution buffer.

## Current engine boundary

The TRUEOS vGPU C-ABI currently accepts only point-list, line-list and
triangle-list for `IndexedDrawBatchV2`. It has no line-loop topology value and
no persistent GPU-side command/selector buffer. The closed loop is therefore
represented truthfully as a three-segment line list, and the CPU-authority
signal still chooses the per-frame `IndexedDrawBatchV2` descriptor. A future
GPU-side selector can consume the same immutable document/buffers without
changing this asset format.

The checked-in vGPU broker also canonicalizes immediate triangle indices to
CCW before resident submission. The authored triangle is already CCW, so the
normalization is a no-op and the intended winding is explicit in the source
document.

This was verified against the checked-in authority, not assumed from driver
convention: `../TRUEOS/src/r/io/vgpu_cabi.rs` maps only point/line/triangle
lists at `broker_primitive_topology`, while `../TRUEOS/src/gpu/vgpu.rs`
rejects incompatible index counts and swaps clockwise immediate triangles to
CCW before resident submission.
