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

The authored document holds four colored copies of the same intentionally CCW
three-point stamp. Picasso also stores the 1,024-vertex `32 x 32` seed plane
used by the primitive experiment. After readback, initialization builds and
uploads one immutable index catalogue containing every primitive assembly mode
exposed by the TRUEOS V2 batch ABI.

| Key | First press | Second press | Third press | Input contract |
| --- | --- | --- | --- | --- |
| `1` | point-list grid | four dotted circles | grid | circle mode reuses the 256 Key 9 ring vertices |
| `2` | line-list grid | four dashed circles | line-list adjacency | circle mode pairs each vertex once, without joins between dashes |
| `3` | line-strip grid | four closed circles | line-strip adjacency | four independent 65-index draws close the 64-vertex circles |
| `4` | triangle list | triangle-list adjacency | triangle list | adjacency is currently a deliberate clear-only frame |
| `5` | triangle strip | triangle-strip adjacency | triangle strip | adjacency is currently a deliberate clear-only frame |
| `6` | triangle fan | triangle fan | triangle fan | repeated presses partition the plane into fans of 1024, 512, 256, 64, 32, 16, then 8 vertices |
| `7` | unbound | — | — | no function |
| `8` | 256-quads gap grid | two gapped quad rings | gap grid | toggles the original 1,024-seed grid and a QUADLIST reinterpretation of Key 9's exact ring vertices |
| `9` | quad strip | rerolled quad strip | rerolled quad strip | two independent thin 64-segment annuli with fresh ±0.15% X/Y vertex jitter; each first inner/outer pair repeats only to close its strip |
| `0` | rectangle list | same | same | two rings of equal screen-space rectangles; three explicit corners, fourth implied |

The vertex and index execution buffers are written once during initialization.
Press `0` through `9` to select topology; Keys `1` through `3` include the
circle interpretations above, while Keys `4` and `5` toggle their ordinary and
adjacency forms. Each frame changes only the draw descriptors.
Point-list dots (`1`) are the startup default. Point-list Key `1` is deliberately
split into two immediate-color draws: source ordinals divisible by three are
green, while the other two ordinals are red. This makes the `green, red, red`
partition visible without changing the XYZ vertex layout or shader ABI. Colors are decoded
from the opaque BMP returned by Picasso and carried as immediate per-draw RGBA,
so the demo no longer depends on the retained sampled-material probe. This
immediate pipeline preserves draw order without an implicit depth surface and
retries transient UI4 or Render0 contention.

## Adjacency availability

The four `_ADJ` selections remain wired to their native topology values, but
currently submit one minimum-size primitive whose indices all name the same
seed. It is intentionally degenerate, so the result is a clear-only frame.
This keeps the controls honest until the geometry-shader contract is known.

vGPU exposes admission as
`DeviceInfo::FLAG_ADJACENCY_TOPOLOGY_RENDERING`. Potato Stamps checks the flag
before rendering; other devices retain the ordinary mode and receive
`ERR_UNSUPPORTED` before resident submission.

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
