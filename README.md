# orthos

Markdown to PDF compiler with LaTeX math support, Lora typography, and Obsidian-inspired styling.

## Features

- **Markdown** — full GFM: headings, lists, tables, blockquotes, code blocks, strikethrough, task lists, footnotes, images
- **LaTeX math** — inline `$...$` and display `$$...$$` with automatic conversion to Typst syntax
- **Typography** — Lora (serif body), JetBrains Mono (code), Latin Modern Math (equations)
- **Styling** — warm page background, clean tables, bordered blockquotes, no heading numbering
- **HTML images** — `<img src="...">` tags are supported as an alternative to Markdown image syntax

## Usage

```bash
orthos input.md -o output.pdf
```

If `-o` is omitted, the output path is derived from the input file (e.g. `input.md` → `input.pdf`).

## Installing

```bash
just install
```

This installs to `~/.local/bin`. Ensure `~/.local/bin` is in your `PATH`.

## Building from source

```bash
just build      # release build
just check      # cargo check
just lint       # cargo clippy
```

## Fonts

Fonts are embedded in the binary at compile time. To fetch or update fonts:

```bash
just fetch-fonts
```

This downloads Lora (variable → static instances with adjusted italic slant), Latin Modern Math, and JetBrains Mono.

## Requirements

- Rust 1.85+ (edition 2024)
- `just` (for the `justfile` recipes)
- Python 3 + fonttools (for `just fetch-fonts`)

## License

MIT
