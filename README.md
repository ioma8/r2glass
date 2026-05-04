# r2glass

**radare2 GUI frontend** — a desktop application built with [egui](https://github.com/emilk/egui) and [r2pipe](https://github.com/radareorg/r2pipe.rs).

Browse disassembly, hex dumps, control-flow graphs, and decompiled output from radare2 through a multi-view interface. Navigate symbols, inspect headers and relocations, set breakpoints, and step through code — all without leaving the GUI.

## Features

- **Multi-view output** — Disassembly, Hex, Graph, Decompile, and Info views
- **Symbol navigation** — Filterable function/symbol list in the left panel
- **Inspector panel** — Quick details and metadata on the right
- **Command console** — Run arbitrary r2 commands and see output
- **Debug controls** — Start, continue, step into/over, breakpoints, registers, backtrace
- **Background jobs** — Non-blocking analysis (full, deep, type analysis)
- **Workspace management** — Save and restore analysis sessions
- **Copyable output** — All text panes are selectable for copy

## Requirements

- **radare2** (`r2`) must be installed and on your `PATH`
- Rust 1.85+ (edition 2024)

## Installation

```bash
# Install from source
cargo install --git https://github.com/your-username/r2glass

# Or build locally
git clone https://github.com/your-username/r2glass
cd r2glass
cargo build --release
./target/release/r2glass
```

## Usage

```bash
# Open a binary
r2glass ./binary

# Show version
r2glass --version

# Help
r2glass --help
```

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `P` | Cycle views (Disasm → Hex → Graph → Decompile → Info) |
| `↑` / `↓` | Seek line up/down |
| `Page Up` / `Page Down` | Seek page up/down |

## Development

```bash
# Run
cargo run -- ./some-binary

# Test
cargo test

# Build release
cargo build --release
```

## Architecture

r2glass runs radare2 in a background worker thread, communicating via r2pipe. The egui UI polls for results asynchronously, keeping the interface responsive during long-running analysis. Background jobs (analysis, decompilation) deliver results through channels and update views on completion.

## License

MIT
