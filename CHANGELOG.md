# Changelog

## [v0.2.0] - 2025-07-27

### ✨ Added

- Support for `--init` flag to automatically create the default directory and a sample `rfortunes.dat` file.
- Cross-platform default path detection:
    - Linux/macOS: `/usr/local/share/rfortune/rfortunes.dat`
    - Windows: `%APPDATA%\rfortune\rfortunes.dat` or fallback to `C:\Users\Public\rfortune\rfortunes.dat`
- Basic CLI argument parsing via `clap`:
    - `--file <path>` to specify a custom `.dat` file
    - `--version`, `--help` standard output
- New `README.md` with full documentation of features, usage, installation, and init.
- Modular refactoring of codebase:
    - `main.rs`: CLI and application entry point
    - `loader.rs`: reads and parses `.dat` files
    - `fortune.rs`: selects a random fortune

### 🛠 Changed

- Default fortune file format now expects phrases separated by `%` on their own line (BSD style).
- Improved error messages and file validation.

---
