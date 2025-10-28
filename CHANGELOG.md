# Changelog

## [0.5.1] - 2025-10-27

### Added

- Embedded Windows application icon (`res/rFortune.ico`) directly into the executable.
    - Implemented using the `winres` build dependency.
    - The icon is now visible in Windows Explorer and taskbar.
- New `res/` directory introduced for graphical assets (SVG, PNG, ICO).

### Changed

- Build process updated to automatically compile and embed Windows resources during `cargo build --release`.

### Notes

- The `.res` file generated during build is temporary and not stored in the repository.

---

## [v0.5.0] - 2025-10-04

### 🔧 Refactoring

- Split CLI definition into a dedicated `cli.rs` module
- Added a new `commands.rs` module to handle subcommand actions (`config init`, `file init`, `cache clear`)
- Extracted configuration logic from `utils.rs` into a new `config.rs` module
- Simplified `main.rs` to only parse CLI input and dispatch commands

### ✨ CLI Improvements

- Introduced subcommands:
    - `config init` → initialize the configuration file
    - `file init`   → create a sample default fortune file (`rfortune.dat`)
    - `cache clear` → clear the cache directory
- Enhanced `--help` output with a detailed `long_about` description and usage examples

### ✅ Misc

- Clearer module boundaries (`cli`, `commands`, `config`, `utils`, `loader`)
- Improved maintainability and readability of the codebase

---

## [v0.3.0] - 2025-07-30

### 🔧 Refactoring

- Removed `fortune.rs` and consolidated logic into new `utils.rs`
- Updated `main.rs` and `lib.rs` to use `rfortune` crate structure consistently

### 🧪 Unit Testing

- Added unit tests for `loader` module (parsing `.dat` files)
- Added unit tests for `utils` module, including `random_quote` and `print_random`
- Fixed the signature of `print_random()` to accept file path for cache handling

### 💾 Cache Support

- Implemented cache system to avoid repeating the same fortune twice
- Cache is stored in user-specific system path (`$XDG_DATA_HOME/rfortune/cache/` or `%APPDATA%\rfortune\cache\`)
- Added tests for reading/writing cache and ensuring uniqueness of quotes
- Introduced `--clear-cache` flag to manually delete the entire cache directory

### ✅ Misc

- Ensured full cross-platform compatibility (Linux, macOS, Windows)
- Cleaned up unused code and improved module boundaries

---

## [v0.2.2] - 2025-07-29

### Changed

- Updated the `README.md` in the `homebrew-rfortune` tap repository to include installation instructions via Homebrew.

### Notes

- No changes to the binary or functionality of `rfortune` itself.

---

## [v0.2.1] - 2025-07-29

### Added

- Support for publishing `rfortune` to [crates.io](https://crates.io/crates/rfortune)
- Updated `Cargo.toml` with metadata required by crates.io:
    - Package description, authors, license, keywords, categories
    - Repository and homepage URLs
    - Included files for packaging

### Notes

- This version does not introduce new features or changes to functionality.
- Users can now install `rfortune` directly via:
  ```bash
  cargo install rfortune
  ```

---

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
