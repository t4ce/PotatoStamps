# Potato Stamps

`Potato Stamps` is a deliberately small Picasso/TRUEOS Blueprint demo. It is
registered in `TRUEOS-Blueprints/apps.json`; the checked-in TRUEOS
`startup.json` launches `potato-stamps.bp` in the `pot` slot.

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

The document holds four colored copies of the same intentionally CCW
three-point stamp. After Picasso readback, initialization builds and uploads
one immutable index catalogue containing every primitive assembly mode exposed
by the TRUEOS V2 batch ABI:

| Mode | Topology | Index sequence |
| --- | --- | --- |
| `1` | point list | `0, 1, 2` |
| `2` | line list (closed) | `0, 1, 1, 2, 2, 0` |
| `3` | line strip (closed) | `0, 1, 2, 0` |
| `4` | triangle list | `0, 1, 2` |
| `5` | triangle strip | `0, 1, 2` |
| `6` | triangle fan | `0, 1, 2` |

The vertex and index execution buffers are written once during initialization.
Press `1` through `6` to select topology; each frame changes only the four draw
descriptors. Triangle list (`4`) is the startup default. Colors are decoded
from the opaque BMP returned by Picasso and carried as immediate per-draw RGBA,
so the demo no longer depends on the retained sampled-material probe. This
immediate pipeline preserves draw order without an implicit depth surface and
retries transient UI4 or Render0 contention.

## Current engine boundary

The TRUEOS vGPU C-ABI and resident command encoder accept point list, line
list/strip, and triangle list/strip/fan for `IndexedDrawBatchV2`. Intel has no
line-loop assembly value, so closed loops explicitly repeat the first index in
a line strip or use three independent line-list segments.

The kernel bounds-checks the Picasso-derived vertex/index buffers and each
draw range, maps the chosen topology to the Intel VF value, encodes the batch,
and submits that command stream through the Picasso GuC carrier. GuC schedules
the already-encoded 3D batch; it does not reinterpret topology itself.

Triangle-list indices are canonicalized to CCW before resident submission;
strip and fan order is preserved because their native assembly semantics
depend on index order.
