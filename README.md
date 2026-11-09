<p align="center">
  <img src="assets/app-icon/hicolor/256x256/apps/ph.pdf.png" width="128" height="128" alt="PDF icon">
</p>

<h1 align="center">PDF</h1>

<p align="center">
  <b>The PDF workbench; an open-source, modern PDF reader and editor, built in pure Rust.</b><br>
  Read, organize, combine, split and secure PDFs in a fast, native app, written in Rust from the ground up.<br>
  macOS · Windows · Linux · FreeBSD · the web
</p>

<p align="center">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-b70228?style=flat-square">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-8a011e?style=flat-square">
  <img alt="Platforms: macOS, Windows, Linux, FreeBSD, web" src="https://img.shields.io/badge/runs%20on-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20FreeBSD%20%C2%B7%20web-b70228?style=flat-square">
  <img alt="No account, no telemetry" src="https://img.shields.io/badge/no%20account-no%20telemetry-8a011e?style=flat-square">
  <br>
  <a href="https://sonarcloud.io/project/overview?id=ngjordan_pdf"><img alt="Scanned by SonarCloud" src="https://img.shields.io/badge/Scanned_by-SonarCloud-4b9fd5?style=flat-square&logo=sonarqubecloud&logoColor=white"></a>
  <a href="https://coderabbit.ai"><img alt="Reviewed by CodeRabbit" src="https://img.shields.io/coderabbit/prs/github/ngjordan/pdf?style=flat-square&label=Reviewed%20by%20CodeRabbit&logo=coderabbit"></a>
</p>

<p align="center">
  <a href="https://iridium.ph"><img alt="Join the community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://iridium.ph/apps/pdf"><b>PDF on iridium.ph</b></a> ·
  <a href="https://iridium.ph/">Iridium Technologies</a> ·
  <a href="https://iridium.ph/apps">All the apps</a>
</p>

<br>

<p align="center">
  <img src="docs/images/iridium-pdf-viewer.png" alt="PDF with the PDF Showcase cover page open, the All tools panel on the left and 20 threaded comments on the right" width="100%">
  <br>
  <sub>The PDF Showcase, a 13-page specimen PDF, open with the All tools panel and threaded comments.</sub>
</p>

> [!NOTE]
> **PDF is built by Iridium Technologies, Inc.** Digital, generative, music,
> games &mdash; if you make things, you're one of us. **[Come say hi on Discord](https://iridium.ph).**

<p align="center">
  <a href="#highlights">Highlights</a> ·
  <a href="#read-anything-beautifully">Read</a> ·
  <a href="#find-it-select-it-copy-it">Find</a> ·
  <a href="#organize-pages-like-cards-on-a-table">Organize</a> ·
  <a href="#combine-and-split-without-losing-a-thing">Combine &amp; split</a> ·
  <a href="#open-protected-documents-and-respect-their-rules">Protect</a> ·
  <a href="#comments-forms-layers-and-attachments">Forms &amp; layers</a> ·
  <a href="#runs-everywhere-stays-yours">Everywhere</a> ·
  <a href="#built-for-agents-too">Agents</a> ·
  <a href="#how-its-built">How it's built</a> ·
  <a href="#get-started">Get started</a> ·
  <a href="#whats-next">What's next</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#the-crafting-apps">Crafting Apps</a>
</p>

---

## Community

PDF is part of [Iridium Technologies](https://iridium.ph). Come say hello, get help and follow development:

- **Discord: [iridium.ph](https://iridium.ph)**. This is the fastest way to get help and share feedback. The app has a Discord button in its title bar.
- **Web page:** [iridium.ph/apps/pdf](https://iridium.ph/apps/pdf)
- **Source:** [github.com/ngjordan/pdf](https://github.com/ngjordan/pdf)

The upstream project name and logos are trademarks of the upstream team and are not open source. They may be used only unmodified, and only as part of PDF (see `docs/brand/LICENSE-brand.txt`). Forks and modified versions must remove them.

## Highlights

<table>
<tr>
<td width="33%" valign="top">

### Faithful
Real-world typography: world scripts, vertical Japanese, colour emoji, gradients, soft masks and transparency. All of it renders the way the author intended.

</td>
<td width="33%" valign="top">

### Fearless
Ordinary saves append your changes and leave the original bytes untouched. Edits to damaged objects whose xref and header generations disagree use a full rewrite, keeping both accepted references connected to the same repaired object. Writes are atomic, undo runs deep, and nothing is lost if you close by mistake.

</td>
<td width="33%" valign="top">

### Yours
No account, no telemetry, no cloud. It works offline and opens instantly. The engine, CLI and app are all open source.

</td>
</tr>
</table>

---

## Read anything, beautifully

PDF renders PDFs with care for the details that make a page feel right: kerning and ligatures, right-to-left and complex scripts, vertical CJK, colour emoji, shadings, blend modes, soft masks and optional content.

<p align="center">
  <img src="docs/images/iridium-pdf-scripts.png" alt="The Scripts of the World page: Arabic, Hebrew, Devanagari, Thai, Greek, Cyrillic, Chinese, Korean, IPA, Armenian, Georgian and Tamil samples, with vertical Japanese in the right margin" width="100%">
  <br>
  <sub>Twelve writing systems on one page, plus vertical Japanese, at 125%.</sub>
</p>

- **Deep zoom stays sharp.** Large pages render in tiles, so text stays crisp at any magnification.
- **Built to survive bad files.** Every page renders in isolation and damaged documents are repaired. Across the 983-file pdf.js test corpus the result is 0 crashes.
- **Layouts for every task:** continuous, single page, two-up, view rotation, full screen and a distraction-free Read mode.
- **Light and dark themes**, both designed to be easy on the eyes for long sessions.

<table>
<tr>
<td width="50%"><img src="docs/images/iridium-pdf-twoup.png" alt="Two facing pages in Read mode with the dark theme: the Foreword and the Setting Text chapter with its drop cap and pull quote"></td>
<td width="50%"><img src="docs/images/iridium-pdf-dark.png" alt="The dark theme showing the Code and Images chapter, a syntax-coloured listing and a fractal image, with the comments panel open"></td>
</tr>
<tr>
<td align="center"><sub>Two-up Read mode, ready for long reading</sub></td>
<td align="center"><sub>The dark theme, with the comments panel open</sub></td>
</tr>
</table>

## Find it, select it, copy it

Search the whole document as you type, step through matches with <kbd>⌘G</kbd>, and select text that comes out in the right reading order. That holds for columns, right-to-left runs and CJK too.

## Navigate long documents

Launch with `iridium-pdf --two-paged file.pdf` (or `iridium-pdf file.pdf --two-paged`) to open each command-line PDF in noncontinuous two-page view. This optional launch override does not change saved default-layout preferences or PDFs restored from the previous session. An explicit `--layout` option takes precedence on the active document regardless of argument order; `--default-layout` retains its existing behavior for subsequently opened documents.

Right-click a PDF tab to close it, reopen the last closed file-backed tab, or reveal its file. Closing still asks about unsaved changes. Reopening reads the saved file from disk; it does not restore discarded edits. Reveal selects the file in Explorer or Finder. On Linux and FreeBSD it requests `org.freedesktop.FileManager1.ShowItems`, falling back to opening the containing directory with `xdg-open` when that service is unavailable. Reopen and Reveal are unavailable on the web; Reveal is also unavailable for pathless documents.

Bookmarks, page thumbnails and the document's own page labels (i, ii, 1, 2…) keep you oriented in long documents. Search bookmark titles in the Bookmarks panel to find nested entries even when their parents are collapsed. Matches keep their ancestors for context; Clear restores the unfiltered tree without changing its expansion state.

In **Inspector > Pages**, use **Thumbnail size** and the smaller/larger buttons to show more pages or inspect larger previews (50–150%). This changes only the thumbnails, keeping multi-page selection and the current page in view. The size belongs to the open document view and survives reloading its file; it is not remembered after restarting. Agents can set it with `ui.set {"key":"thumbnail-zoom","value":"75"}` and read `active.thumbnail_zoom` from `ui.state`.

<table>
<tr>
<td width="50%"><img src="docs/images/iridium-pdf-find.png" alt="The find bar showing match 10 of 16 for the word 'type', highlighted in the Expressive Type chapter heading"></td>
<td width="50%"><img src="docs/images/iridium-pdf-bookmarks.png" alt="The Bookmarks panel showing the nested outline of the showcase, with page labels such as Cover, i and ii, next to the Scripts of the World page"></td>
</tr>
<tr>
<td align="center"><sub>Find as you type: match 10 of 16</sub></td>
<td align="center"><sub>Nested bookmarks with the document's own page labels</sub></td>
</tr>
</table>

---

## Organize pages like cards on a table

Open **Organize pages** to see every page at once:
- **Select pages:** click, <kbd>⌘</kbd>-click or <kbd>⇧</kbd>-click.
- **Change them:** rotate, delete, insert blank pages, insert pages from another file, and move them earlier or later.
- **Undo anything:** <kbd>⌘Z</kbd>, then save.

<p align="center">
  <img src="docs/images/iridium-pdf-organize.png" alt="The Organize pages grid with the showcase's pages as thumbnails, three of them selected, and the page toolbar above" width="100%">
  <br>
  <sub>Organize pages with three pages selected and the page tools in the toolbar above.</sub>
</p>

<table>
<tr>
<td width="50%" valign="top">

**Undo that goes the distance.** Each change is one step in a history you can walk backwards and forwards. The Edit menu names the step ("Undo Rotate pages"), and undo still works after you save.

**Saves you can trust:**
- *Incremental:* the original bytes stay byte-for-byte intact.
- *Atomic:* the file is written to a temporary copy, then swapped in.
- *Verified:* independently checked with qpdf, on every CI run.

When open documents exceed the window width, scroll over the tab strip with the mouse wheel or trackpad, or use its horizontal scrollbar. Opening or switching to a document brings its tab into view.

Unsaved documents carry a dot on their tab, and closing or quitting asks before anything is lost. Changes are autosaved every minute. If PDF ever quits unexpectedly, it offers to recover your work the next time it opens. Encrypted documents stay encrypted on disk.

</td>
<td width="50%" valign="top"><img src="docs/images/iridium-pdf-split.png" alt="The Split document dialog over the organize view, set to one page per file and reporting that it creates 13 files from 13 pages"><br><sub>Split document: one page per file makes 13 files.</sub></td>
</tr>
</table>

## Combine and split without losing a thing

**Combine files** merges any number of PDFs into one. Each file gets a bookmark, with its own bookmarks nested underneath.

**Extract** copies the pages you select into a new document. **Split** divides a document every *n* pages, or before the pages you choose.

Nothing quietly disappears along the way:
- links and named destinations are rewired to the copied pages;
- form fields stay interactive;
- layers keep their on/off defaults;
- attachments come along.

Every page of a combined document renders pixel-identical to its source.

```sh
iridium-pdf-cli combine report.pdf appendix.pdf --out combined.pdf
iridium-pdf-cli extract report.pdf --pages 1,3,5-8 --out highlights.pdf
iridium-pdf-cli split   report.pdf --every 10 --out-dir parts/
```

---

## Open protected documents and respect their rules

PDF implements the PDF standard security handler completely:
- every revision, from 40-bit RC4 to AES-256;
- user and owner passwords, including Unicode passwords normalised with SASLprep;
- crypt filters and attachment-only encryption.

Documents restricted by their author show a clear notice, and PDF honours their permissions. Enter the owner password and the restrictions lift. Edits to encrypted documents are saved encrypted, under the same keys.

<table>
<tr>
<td width="50%" valign="top"><img src="docs/images/iridium-pdf-properties.png" alt="The Document Properties dialog on its Description tab, with editable title, author, subject and keywords, and tabs for Security, Fonts and Advanced"><br><sub>Document Properties, Description tab</sub></td>
<td width="50%" valign="top">

**Document Properties** shows:
- the document's title, author, subject and keywords, which you can edit;
- the fonts it uses and whether each is embedded;
- PDF version, page size, tags, fields, layers and attachments;
- the full security picture: encryption method, which password opened it, and each permission.

</td>
</tr>
</table>

## Comments, forms, layers and attachments

<table>
<tr>
<td width="50%"><img src="docs/images/iridium-pdf-forms.png" alt="The Interactive Form page with highlighted text fields, checkboxes, radio buttons, a list and a signature field, and the Fields panel listing all 13 fields and their values"></td>
<td width="50%"><img src="docs/images/iridium-pdf-layers.png" alt="The Review and Markup page with highlights, shapes, ink and an APPROVED stamp under a DRAFT watermark, and the Layers panel with Draft watermark and Print-only notes"></td>
</tr>
<tr>
<td valign="top"><b>Forms</b>: every field with its current value, field highlighting, and checkboxes, radio buttons, lists and signatures drawn the way their author designed them.</td>
<td valign="top"><b>Layers</b>: switch optional content on and off and the page re-renders instantly. <b>Comments</b> appear as threaded conversations, and <b>attachments</b> can be opened or saved.</td>
</tr>
</table>

## Scan pages, then read the words back before trusting them

**Scan & OCR ▸ Recognize text** turns a scan into searchable text — the current page, a range, or a folder of files written as searchable copies — with optional cleanup before reading (auto-rotate, deskew, denoise, binarize; word boxes map back to the original scan) and a second engine: an installed Tesseract joins the built-in one, merged per your strategy. A searchable result applies as one undoable step; the **Editable text** output opens the **OCR Verify screen** instead, where each word carries the engine's confidence as a colour, suspects are one keypress away, and corrections, a user dictionary and Re-OCR of a region or a page happen before Accept writes the new document. **Scan & OCR ▸ Correct recognized text** reviews any recognition there before it is applied.

<p align="center">
  <img src="docs/images/iridium-pdf-ocr-verify.png" alt="The OCR Verify screen: a scanned note with each recognized word boxed in its confidence colour, the info strip reporting a 95% average confidence and one low-confidence word, and the confidence legend with its thresholds below" width="100%">
  <br>
  <sub>The OCR Verify screen: words coloured by confidence — the legend spells the thresholds out — reviewed and corrected before anything is applied.</sub>
</p>

---

## Every tool, one keystroke away

Press <kbd>⌘K</kbd> to search every tool and command, or browse the **All tools** catalogue. Tools that are still in development are marked with the milestone that will ship them.

<table>
<tr>
<td width="50%"><img src="docs/images/iridium-pdf-palette.png" alt="The command palette searching for 'page', listing Page grid, Page labels, Rotate pages, Insert pages, Delete pages, Extract pages and more, each with the tool it belongs to"></td>
<td width="50%"><img src="docs/images/iridium-pdf-tools.png" alt="The Welcome to PDF home screen with recommended tools, a recent file, a privacy note, and the full tool catalogue in the side panel"></td>
</tr>
<tr>
<td align="center"><sub>The <kbd>⌘K</kbd> command palette</sub></td>
<td align="center"><sub>The home screen and the All tools catalogue</sub></td>
</tr>
</table>

---

## Runs everywhere, stays yours

- **Native on macOS, Windows, Linux and FreeBSD**, and **in the browser** through WebAssembly, from the same Rust codebase. Windows builds come for x64, x86 and ARM64 (Windows on ARM, no emulation); every ARM64 change is tested on ARM64 hardware in CI.
- **Private by design.** Documents never leave your machine. There's no account, no telemetry and no cloud processing.
- **Engine first.** Parsing, rendering and editing live in reusable library crates. The interface is one swappable layer on top.
- **Scriptable.** The `iridium-pdf-cli` tool (see [Built for agents, too](#built-for-agents-too)) covers inspecting, rendering, extracting text, editing, combining, extracting pages and splitting. Robustness sweeps run on the same engine as the app.

```sh
iridium-pdf-cli info  form.pdf                                  # structure as JSON
iridium-pdf-cli text  paper.pdf --page 3                        # reading-order text
iridium-pdf-cli edit  in.pdf --rotate 1,2:90 --delete 5 --title "Q3" --out out.pdf
```

---

## Built for agents, too

Every engine feature is reachable without the GUI, through one table of JSON-Schema-described tools: open, inspect, render pages to PNG, extract and find text, rotate, delete, move and insert pages, edit bookmarks and page labels, add, reply to, restyle and delete comments (highlight a phrase just by naming it), list and fill in form fields, protect with passwords, set metadata, undo and redo, save, combine, extract and split. Three front doors share it:

- **`iridium-pdf-cli run`**, for one-off calls and JSON scripts:

  ```sh
  iridium-pdf-cli tools                                        # every tool and its JSON Schema
  iridium-pdf-cli run text_find doc=1 query=invoice            # key=value; values parse as JSON
  iridium-pdf-cli run --script review.json                     # e.g. comment_add {"type": "highlight", "find": "total due"}
  iridium-pdf-cli run --script steps.json --root ./work        # several steps in one session
  iridium-pdf-cli run --script steps.json --report             # one JSON document: per step its number, tool, status, result or error
  ```

  With `--root`, every file the script's steps read or write stays in that directory, including the PNG a step saves with `"out"` (the script itself is read from wherever you name it).

- **An MCP server**, for AI agents such as Claude. **It is opt-in:** PDF never starts it on its own, and it opens no network port. It runs only while an agent launches `iridium-pdf-cli mcp`, talks over stdin/stdout, and stops when the agent disconnects. To enable it, add it to your agent's MCP configuration:

  ```json
  { "mcpServers": { "iridium-pdf": { "command": "iridium-pdf-cli", "args": ["mcp", "--root", "/path/to/your/pdfs"] } } }
  ```

  `--compact` shrinks the tool list the agent has to read: `tools/list` returns about ten core tools plus `tool_search` and `tool_call`, which find and run every other tool, so the list costs far fewer tokens. Every tool still works.

  `--root` confines every file the agent can read or write to one directory. Builds that should not include the server at all can use `cargo build -p iridium-pdf-cli --no-default-features`.

- **The Rust API** (`iridium-pdf_automation::Automation::call`), for embedding.

Edits stay in memory, undoable, until `doc_save`. Saving to the same file appends an incremental update, so the original bytes are preserved, and the write is atomic. Unsaved changes are never discarded silently.

### Driving the app itself

Start the desktop app with `iridium-pdf --control ~/.iridium-pdf-control.json` and an agent can see and operate the real interface: the widget tree with labels and positions (from the accessibility tree), clicks, typing, keys, commands, view options and screenshots. This is also off by default. It listens only on loopback, and every connection must present the random token written to that file, which only you can read.

Keep the control file in a folder only you can write, not a shared one such as `/tmp`: another user could create the file there first and receive your commands. `iridium-pdf-cli ui` refuses a control file that is a symbolic link, or one that another user owns, or that another user or a group such as Administrators can read or write: on macOS, Linux and FreeBSD by its owner and mode (the app writes it with mode 600), on Windows by its owner and access list (the app gives it one with only you and SYSTEM on it). The app doesn't start if it can't write the file. On Windows, cmd.exe doesn't expand `~`: use `%USERPROFILE%\.iridium-pdf-control.json` there.

```sh
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json inspect query=rotate      # find widgets
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json click label="Organize pages"
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json key key=K modifiers='["command"]'
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json command id=comment.square   # pick a tool, then draw:
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json drag from='[400,300]' to='[600,420]'
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json screenshot --out window.png
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json resize width=1200 height=800
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json focus label="Read"
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json inspect focused=true
iridium-pdf-cli ui --control ~/.iridium-pdf-control.json quit
```

`ui.panel` opens, closes or sizes a side panel: `{"panel":"tools"|"side","open":true|false,"width":320}`.
`tools` is the Tools panel and `side` the panel the rail opens (reopening it shows the one open last).
`width` is clamped to the panel's limits (Tools 240–520, side panel 260–520 points). `ui.dock.reset`
resets the side panels: Tools open on the left, default widths. The control channel accepts these
methods directly; `ui.command` also accepts `{"id":"ui.panel","params":{...}}`. Saved preferences keep
the Tools side, the open panels and their widths. Invalid requests return an error without changing
the document or the panels.


`resize` requests a native inner-window size in logical points (each dimension must be between 320 and 8192); the window manager may constrain it. `state` reports the observed `window.width`, `window.height` and `window.pixels_per_point`. `focus` accepts exactly one widget `id` or unique enabled, focusable `label`; subsequent `key` requests traverse or activate the actual widgets. `inspect` includes focusability and accepts a `focused` filter. `quit` takes no parameters: clean documents exit normally, while unsaved edits or typing open the existing Save/Don't save/Cancel prompt and return `quitting: false, needs_confirmation: true`. It never silently discards work. Window resizing and application quit are native-only.

---

## How it's built

PDF is a Cargo workspace of focused crates, layered so the core never depends on the UI:

| Crate | What it does |
|---|---|
| `iridium-pdf-filters` | Every PDF stream filter (Flate, LZW, ASCII85, RunLength, predictors), encode and decode, property-tested |
| `iridium-pdf-crypt` | The standard security handler: RC4, AES-128/256, revisions 2–6, permissions |
| `iridium-pdf-cos` | The PDF object layer: tolerant parsing, repair, copy-on-write edits, incremental and full writing |
| `iridium-pdf-organize` | Page operations, combine / extract / split, bookmarks, page labels, document information |
| `iridium-pdf-fonts` | Font metrics and encodings for generated appearances |
| `iridium-pdf-annot` | Comments: builders and appearance streams for notes, text markup, shapes, ink and text boxes; replies, status, edits |
| `iridium-pdf-forms` | Interactive forms: the field model, filling with regenerated appearances, Clear form |
| `iridium-pdf-render` | Rendering, inspection and text extraction with reading order |
| `iridium-pdf-engine` | The façade every frontend uses: sessions, edits, undo, saving, the tool catalogue |
| `iridium-pdf-automation` | Agent control: the headless tool table, `iridium-pdf-cli run`, and the opt-in MCP server |
| `iridium-pdf-ui-egui` | The desktop and web interface |

**Quality gates.** Every change passes the same automated checks:
- formatting, and Clippy with warnings as errors;
- 200+ unit, property and UI tests;
- crate-layering rules and a WebAssembly build check;
- an asset-licence audit.

On top of those, two corpus sweeps run over real-world files:
- **Opening and rendering:** of the 983 pdf.js test files, 963 open and render cleanly, with 0 crashes.
- **Open, edit and save round trips:** 958 succeed.

The output is verified against independent implementations: hayro and qpdf on every CI run, and
poppler over the corpus on demand (`cargo xtask text-oracle`).

PDF is a clean-room implementation. Its behaviour comes from the ISO 32000 specification and black-box observation, never from anyone else's code. Every icon, font and image is openly licensed and listed in [ATTRIBUTION.md](ATTRIBUTION.md).

## Get started

Building needs Rust 1.95 or later.

```sh
git clone https://github.com/ngjordan/pdf
cd iridium-pdf
cargo xtask models                                # fetch the OCR models Scan & OCR needs (release builds include them)
cargo run --release -p iridium-pdf -- some.pdf     # desktop app
cargo xtask demo-pdf                              # build the showcase PDF used in these screenshots
cargo xtask screenshots                           # regenerate every screenshot in this README
```

The interface language is chosen in **Menu → Edit → Preferences…** (Command-comma on macOS,
Ctrl-comma elsewhere, also with no document open; Auto follows the system language;
see [docs/localization.md](docs/localization.md)) and saved. Japanese covers commands,
dialogs, panels and keyboard shortcuts. Command search accepts the translated label, the
English label and the stable command id; filenames, PDF contents, author names, custom
action names and error details from the engine or the operating system keep their own text.

Japanese fonts come from [craft-fonts](https://github.com/ngjordan/craft-fonts), an optional build
input that every release includes. To build with them (Japanese interface text, and Japanese text in
edited PDFs):

```sh
git clone https://github.com/ngjordan/craft-fonts ../craft-fonts
CRAFT_FONTS_DIR=../craft-fonts cargo run --release -p iridium-pdf -- some.pdf
```

Each [GitHub release](https://github.com/ngjordan/pdf/releases) has ready-made builds for macOS,
Windows, Linux (AppImage, Flatpak, `.deb`, `.rpm` and a tarball), FreeBSD and the web; see
[Downloads](#downloads). On Gentoo, the community [::snakebyte
overlay](https://github.com/switch87/snakebyte-overlay) packages the Linux release as
`app-text/iridium-pdf-bin` (not maintained by the PDF team):

```sh
eselect repository add snakebyte git https://github.com/switch87/snakebyte-overlay.git
emaint sync -r snakebyte
echo 'app-text/iridium-pdf-bin ~amd64' >> /etc/portage/package.accept_keywords/iridium-pdf
emerge --ask app-text/iridium-pdf-bin
```

Logs, environment variables and other development notes are in [docs/development.md](docs/development.md).

## What's next

PDF is young and moving fast. The aim is a workbench where you can view, organize, annotate, fill, sign and edit PDFs, for professional and everyday workflows.

**Where it stands (October 2026), honestly:** PDF is **alpha**. Most essential offline PDF features exist in some form (57% of the features tracked in [docs/target-app-parity.md](docs/target-app-parity.md), 94% of core essentials). For everyday work (viewing, commenting, filling and signing forms, organizing pages) readiness is about 55%; across all professional workflows, about 49%; for someone who only needs the essentials, about 66%. Every everyday workflow can be completed, but converting and sharing is weak: no office-document-to-PDF, and .docx export that loses layout.

- **Good today:** viewing and search; organizing, combining and splitting; most kinds of comment; filling and authoring forms (with sandboxed JavaScript); passwords, redaction and sanitizing; basic digital signatures; printing; the Accessibility Checker; agent control through the CLI and MCP.
- **Still borrowed:** pages are drawn by the `hayro` crate while our own renderer is built.
- **Thin or missing:** reliable editing of existing text (especially CJK and right-to-left), OCR beyond Latin script, office document import/export, signature tokens and online long-term validation, Preflight and PDF/X/UA, printing from the web build, physical printer fidelity, and full XFA forms.
- **Hardening:** fuzzing still turns up crashes and hangs on hostile files; each one is fixed with a regression test. Quality has not yet been benchmarked side by side.

**Next, in order:** office-document-to-PDF and a .docx export that keeps layout, the reported bugs and the Chinese interface font, a fidelity harness against a reference renderer, editing existing content, then professional workflows (OCR, office formats, Preflight, signatures, accessibility tagging) and 1.0 polish.

The summary is in **[ROADMAP.md](ROADMAP.md)**; the full assessment by dimension and area is in [docs/target-app-parity.md](docs/target-app-parity.md), the ranked gaps in [docs/gaps.md](docs/gaps.md), and the milestones in [docs/roadmap.md](docs/roadmap.md).

---

## Downloads

**New to PDF?** Download it from the [PDF page on iridium.ph](https://iridium.ph/apps/pdf). That's the easiest way to install it.

**Want a specific build or format?** On GitHub, the [latest release](https://github.com/ngjordan/pdf/releases/latest) has every build listed below, and [all releases](https://github.com/ngjordan/pdf/releases) has earlier versions and their notes. `<ver>` in the file names is the version number, and `SHA256SUMS.txt` lists a checksum for every file.

### Windows

| Build | Installer | Portable |
|---|---|---|
| x64 (64-bit Intel/AMD) | `iridium-pdf-<ver>-windows-x64.msi` | `iridium-pdf-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon and other ARM PCs) | `iridium-pdf-<ver>-windows-arm64.msi` | `iridium-pdf-<ver>-windows-arm64-portable.zip` |
| x86 (32-bit) | `iridium-pdf-<ver>-windows-x86.msi` | `iridium-pdf-<ver>-windows-x86-portable.zip` |

Installers and executables are code-signed.

The portable zip runs from any folder, a USB stick included. Its `portable.txt` keeps the settings,
logs and crash recovery in a `PDFData` folder next to `iridium-pdf.exe`, so nothing is written to
`%APPDATA%`; delete that file to use the normal per-user folders.
Windows doesn't offer the portable copy as a default PDF app on its own, since it registers nothing;
`portable.txt` has the PowerShell lines that register it for your user only, and how to undo them.

The MSI installs for your account only by default, without administrator privileges. Choose
"Everyone on this computer" on its first page, or pass `ALLUSERS=1` from an elevated terminal, to install
for all users. It adds "Edit with PDF"
and "Combine files in PDF" (select several PDFs: they land in the Combine files tab of the
running PDF) to the right-click menu of PDF files and "Create PDF with PDF…" to the right-click menu of
images (.png .jpg .jpeg .tif .tiff .gif .bmp .jp2 .j2k .jpx) in File Explorer (on Windows 11, under
"Show more options"), without changing which app opens them by default. For unattended deployment
without a desktop shortcut (add `ALLUSERS=1`, from an elevated terminal, for an all-users install):

```powershell
msiexec /i "iridium-pdf-<ver>-windows-x64.msi" /qn /norestart INSTALLDESKTOPSHORTCUT=0
```

| Property | `=0` leaves out |
|---|---|
| `INSTALLDESKTOPSHORTCUT` | the desktop shortcut |
| `INSTALLPDFCONTEXTMENU` | "Edit with PDF" and "Combine files in PDF" on PDF files |
| `INSTALLIMAGECONTEXTMENU` | "Create PDF with PDF…" on images |

All three are on for a first install. An upgrade keeps what the installed version has: a shortcut
or menu entry that is there stays, and one that is missing (left out at install, or a desktop
shortcut someone deleted) stays missing, so you don't need to pass them again. A value on the
command line always wins: `=1` puts one back and `=0` removes it. Upgrading from a release that
didn't have a menu yet (0.6.0 and earlier for the PDF menu, 0.2.1 and earlier for the image menu)
adds it, as a first install would. Use the MSI for your architecture. Per-user installation
overrides are not supported.

To install somewhere other than the default `PDF` folder, change the folder on the
welcome screen (type it, or Browse…) or pass it on the command line:
`INSTALLFOLDER="D:\Apps\PDF"`. It is installed for whoever the first screen chose (`ALLUSERS=1`
on the command line for everyone). An upgrade keeps the folder the installed version uses unless
the command line gives another one.

**Pin the checksum, not just the version.** Release candidates of a version share its MSI version
(MSI versions are numbers only, so 1.2.3-rc.1 and 1.2.3 are both 1.2.3), and the installer lets an
MSI of the same version replace the installed one without asking. Two different `0.7.1` MSIs can't
be told apart by version, so if your deployment tool checks the file, pin its SHA-256 from
`SHA256SUMS.txt` (`Get-FileHash <file> -Algorithm SHA256`).

**Signing identities.** The Windows MSIs and both executables are signed through Azure Trusted
Signing with a certificate issued to `Learning Machines Inc` (Atlanta, Georgia, US). The macOS app
and command-line tool are signed with the Developer ID `Learning Machines LLC (DJ6XS33FX8)` and
notarized. The MSI's manufacturer, shown as the publisher in Installed apps, is
`Learning Machines LLC`.

### macOS

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `iridium-pdf-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `iridium-pdf-cli-<ver>-macos-universal.zip` | Signed and notarized |

### Linux

| Format | x86_64 | aarch64 (ARM64) | Notes |
|---|---|---|---|
| AppImage | `iridium-pdf-<ver>-linux-x86_64.AppImage` | `iridium-pdf-<ver>-linux-aarch64.AppImage` | Runs anywhere; updates itself with [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`.zsync` files) |
| Flatpak | `iridium-pdf-<ver>-linux-x86_64.flatpak` | `iridium-pdf-<ver>-linux-aarch64.flatpak` | Sandboxed; `flatpak install --user <file>` |
| Debian/Ubuntu | `iridium-pdf-<ver>-linux-x86_64.deb` | `iridium-pdf-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `iridium-pdf-<ver>-linux-x86_64.rpm` | `iridium-pdf-<ver>-linux-aarch64.rpm` | |
| Tarball | `iridium-pdf-<ver>-linux-x86_64.tar.gz` | `iridium-pdf-<ver>-linux-aarch64.tar.gz` | Unpack anywhere |
| Command-line tool | `iridium-pdf-cli-<ver>-linux-x86_64.tar.gz` | `iridium-pdf-cli-<ver>-linux-aarch64.tar.gz` | `iridium-pdf-cli` alone (and its opt-in MCP server), for servers, CI and agents |

RISC-V (riscv64) builds come as tarballs only: `iridium-pdf-<ver>-linux-riscv64.tar.gz` and `iridium-pdf-cli-<ver>-linux-riscv64.tar.gz`. They are cross-compiled and need glibc 2.39 or newer (Ubuntu 24.04+, Debian 13+).

Every other Linux build needs glibc 2.35 or newer (Ubuntu 22.04+, Debian 12+, Fedora 36+, RHEL 10).

### FreeBSD

| Build | File |
|---|---|
| x86_64 | `iridium-pdf-<ver>-freebsd-x86_64.tar.gz` |

### Web (WebAssembly)

| Build | File | Notes |
|---|---|---|
| Static site | `iridium-pdf-web-<ver>.zip` | Runs in a modern browser; host it on any static server |

---

## The Crafting Apps

PDF is one of five open-source desktop applications, each written from scratch in Rust and
each able to stand on its own.

| | App | What it's for | Code | Learn more |
| <img src="https://raw.githubusercontent.com/ngjordan/sheets/main/assets/app-icon/hicolor/64x64/apps/ph.sheets.png" alt="" width="32" height="32"> | **Sheets** | Spreadsheets and data | [GitHub](https://github.com/ngjordan/sheets) |
| <img src="https://raw.githubusercontent.com/ngjordan/docs/main/assets/app-icon/hicolor/64x64/apps/ph.docs.png" alt="" width="32" height="32"> | **Docs** | Word processing and documents | [GitHub](https://github.com/ngjordan/docs) |
| <img src="https://raw.githubusercontent.com/ngjordan/cad/main/assets/app-icon/hicolor/64x64/apps/ph.cad.png" alt="" width="32" height="32"> | **CAD** | Computer-aided design and drafting | [GitHub](https://github.com/ngjordan/cad) |
| <img src="https://raw.githubusercontent.com/ngjordan/decks/main/assets/app-icon/hicolor/64x64/apps/ph.decks.png" alt="" width="32" height="32"> | **Decks** | Presentations and slide shows | [GitHub](https://github.com/ngjordan/decks) |
| <img src="https://raw.githubusercontent.com/ngjordan/pdf/main/assets/app-icon/hicolor/64x64/apps/ph.pdf.png" alt="" width="32" height="32"> | **PDF** | Reading, organizing and protecting PDF files | [GitHub](https://github.com/ngjordan/pdf) |

It is one of five open-source desktop applications built by Iridium Technologies, Inc.

<br>

<p align="center">
  <a href="https://iridium.ph"><img alt="Join the community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">Come make things with us</h3>

<p align="center">
  Our Discord is where artists of every kind hang out: people who paint, shoot, draw, cut film,
  set type, and people still figuring out what they like to make. Share what you're working on,
  ask for help, tell us what's broken, or tell us what you wish these tools could do.
  Whatever your medium and however long you've been at it, you're welcome here.
</p>

<p align="center">
  <a href="https://iridium.ph"><b>iridium.ph</b></a> ·
  <a href="https://iridium.ph/">iridium.ph</a> ·
  <a href="https://iridium.ph/apps">The Crafting Apps</a> ·
  <a href="https://iridium.ph/apps/pdf">PDF</a>
</p>

---

## License and credits

PDF is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 Iridium Technologies, Inc. and the PDF contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md). Release builds also embed
the Japanese fonts of [craft-fonts](https://github.com/ngjordan/craft-fonts/blob/main/ATTRIBUTION.md)
(SIL Open Font License 1.1).

The upstream project name, wordmark and logos are trademarks of the
upstream team and are not covered by this license. They may be used only unmodified, and only as
part of this repository and PDF, under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
Forks and modified versions must remove them.

<p align="center">
  <a href="https://iridium.ph/"><img alt="PDF" src="assets/app-icon/hicolor/128x128/apps/ph.pdf.png" width="28"></a><br>
  <sub>Made by the <a href="https://iridium.ph/">Iridium Technologies</a> team and community.</sub>
</p>

The Tools panel sits on the left, or on the right with Preferences ▸ Tools panel side. The rail stays at the right edge and opens one side panel beside it. Drag a panel's inner edge to resize it; widths are remembered. F4 shows or hides the side panel and Shift+F4 the Tools panel (also in the View menu); View ▸ Reset side panels restores the default arrangement.
