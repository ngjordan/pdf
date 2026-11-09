# PDF app icon

<img src="iridium-pdf-small.svg" alt="PDF app icon: an engraved lion's head on red" width="128">

**Creature:** a lion, in a frontal head-and-shoulders portrait, mane running off the bottom of the tile.

**Style:** an engraving (woodcut-weight line work) portrait in the Crafting Apps "owl template" framing:
a full-bleed colour field, no frame or roundel, the animal looking at the viewer and filling the tile.

**Palette:** exactly three colours.

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | line work and the figure's contour |
| Paper | `#efe9dc` | the figure (the lion's silhouette) |
| PDF red (app colour) | `#b70228` | the full-bleed field |

**Tile:** `viewBox="0 0 512 512"`, a rounded square with `rx=112` that clips everything. Windows and Linux
icons use the full-bleed tile. macOS icons put it on Apple's grid (an 824 px body centred on a transparent
1024 px canvas).

**Provenance:** the project owner's original drawing, made in the upstream project (2880 px, keyed to the palette), then
vectorised (potrace; no filtering or warping).
Licence: [LICENSE.txt](LICENSE.txt) (`MIT OR Apache-2.0`, like the repo).

## Files

| File | What it is |
|---|---|
| `iridium-pdf.svg` | the master vector (traced at 2048 px); every PNG, `.ico` and `.icns` is rendered from it |
| `iridium-pdf-small.svg` | a lighter vector (traced at 1024 px) for places where size matters, such as this README |
| `iridium-pdf-1024.png` | 1024 px on Apple's grid; also the runtime Dock icon on macOS |
| `iridium-pdf.icns` | macOS icon (16–1024 px) |
| `iridium-pdf.ico` | Windows icon (16–256 px), embedded in `iridium-pdf.exe` by `apps/iridium-pdf/build.rs` |
| `iridium-pdf-document.svg` | Windows PDF file icon master: a Lucide document outline, red badge and outlined PDF lettering; no font dependency |
| `iridium-pdf-document.ico` | Windows PDF file icon (16–256 px), embedded as resource 2 in `iridium-pdf.exe` |
| `hicolor/<n>x<n>/apps/ph.pdf.png` | Linux hicolor theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/ph.pdf.svg` | Linux scalable icon (copy of the master) |

Where it shows: `apps/iridium-pdf/src/main.rs` sets the window icon (Dock, taskbar, Alt-Tab, launcher) and the
Wayland app id `ph.pdf`; `packaging/linux/ph.pdf.desktop` names the
hicolor icon.

Windows PDF files use resource 2 (`"iridium-pdf.exe",-2`) when PDF is chosen as the default
reader; the app keeps resource 1. The document outline is adapted from Lucide `file-text`
1.49.0 (ISC); its badge and outlined lettering are original (`MIT OR Apache-2.0`), covered by
[LICENSE-document.txt](LICENSE-document.txt). Both icons are embedded, requiring no runtime
sidecar. This provides a file type icon; first-page thumbnails still need a thumbnail provider.

## Regenerate

```sh
packaging/icons.sh        # needs resvg and python3; iconutil (macOS) for the .icns
cargo xtask assets        # then update the sha256 values in ATTRIBUTION.toml and run with --write
```
