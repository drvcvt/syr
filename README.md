# syr

**s**ave **y**our **r**ust · **s**trip **y**our **r**elics · **s**ingle-**y**ield-**r**enamer · **s**ick **y**oung **r**efactorer

A CLI tool that automatically manages unused variable prefixes in Rust files by renaming them with underscores.

> 📖 [Deutsche Version](README.de.md)

## What it does

`syr` analyzes your Rust code and automatically:
- Prefixes unused local variables with `_` (e.g., `foo` → `_foo`)
- Removes the `_` prefix when a variable becomes used (e.g., `_bar` → `bar`)
- Reformats your code with `prettyplease`

## Quick Start

### Installation

#### From source
```bash
git clone https://github.com/drvcvt/syr
cd syr
cargo install --path .
```

#### From GitHub
```bash
cargo install --git https://github.com/drvcvt/syr
```

### Usage

Process a single Rust file:
```bash
syr path/to/file.rs
```

The tool will:
1. Parse the file as an AST
2. Analyze variable declarations and usages
3. Rename variables based on usage:
   - `_name` if never used
   - `name` if used
4. Rewrite all bindings and expression paths
5. Format and save the file

## Features

- **Smart renaming**: Variables with the same basename (`foo`, `_foo`, `__foo`) are treated as one logical group
- **Idempotent**: Running multiple times produces stable results
- **Simple**: Operates on a single file at a time
- **Exit codes**: Returns 0 on success, non-zero on errors

## Important Limitations

- **Comments are not preserved**: `syn` removes comments during parsing, and `prettyplease` cannot restore them
- **Simple identifiers only**: Complex macro contexts are not fully supported

## How it Works

The renaming is based on "basenames" - all variants of a variable (`foo`, `_foo`, `__foo`) are logically grouped together. The tool then decides the appropriate form based on usage:

- If a variable is declared but never used → prefix with `_`
- If a variable is used → remove `_` prefix
- Variables already correctly prefixed are unchanged

## License

MIT License - see [LICENSE](LICENSE) for details

Copyright (c) 2025 Matti
