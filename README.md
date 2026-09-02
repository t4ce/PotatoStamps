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
three-point stamp. Picasso also stores the 1,000-vertex `40 x 25` seed plane
used by the primitive experiment. After readback, initialization builds and
uploads one immutable index catalogue containing every primitive assembly mode
exposed by the TRUEOS V2 batch ABI.

| Key | First press | Second press | Input contract |
| --- | --- | --- | --- |
| `1` | point list | point list | seed ordinal modulo 3: green, red, red, repeating |
| `2` | line list | line-list adjacency | both consume the identical 1,000-index range: 500 ordinary pairs versus 250 groups of `adj0, line0, line1, adj1` |
| `3` | line strip | line-strip adjacency | at least two ordinary inputs; adjacency adds one endpoint before and after the visible strip |
| `4` | triangle list | triangle-list adjacency | ordinary groups of three; adjacency groups of six with visible vertices in slots 0, 2, and 4 |
| `5` | triangle strip | triangle-strip adjacency | at least three ordinary inputs; adjacency requires at least six inputs and visible vertices in even slots |
| `6` | triangle fan | triangle fan | repeated presses cycle independent fans of 5, 10, 25, 50, 125, 250, or 1000 vertices |
| `7` | unbound | — | no function (the former ten-vertex interpretation is on key 6) |
| `8` | quad list | same | groups of four |
| `9` | quad strip | same | four independent strips |
| `0` | rectangle list | same | screen-space groups of three |

The vertex and index execution buffers are written once during initialization.
Press `0` through `9` to select topology; keys `2` through `5` toggle their
ordinary and adjacency forms. Each frame changes only the draw descriptors.
Point-list dots (`1`) are the startup default. Point-list Key `1` is deliberately
split into two immediate-color draws: source ordinals divisible by three are
green, while the other two ordinals are red. This makes the `green, red, red`
partition visible without changing the XYZ vertex layout or shader ABI. Colors are decoded
from the opaque BMP returned by Picasso and carried as immediate per-draw RGBA,
so the demo no longer depends on the retained sampled-material probe. This
immediate pipeline preserves draw order without an implicit depth surface and
retries transient UI4 or Render0 contention.

## Adjacency availability

The four `_ADJ` modes are not merely different index grouping. The resident
path also installs checked-in geometry-shader and URB state captured on the
`0xA780` revision `0x04` RPL-S UHD 770. TRUEOS explicitly admits that target
and the physical `0x4680` revision `0x0C` ADL-S rig as gfx120 Xe-LP targets.
The line GS ignores the adjacency-only outer vertices and emits only the
central pair.

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
