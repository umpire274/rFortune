<h1 style="text-align: left;">
  <img src="res/rfortune.svg" width="90" style="vertical-align: middle; margin-right: 8px;" alt="rFortune Logo"/>
  rFortune
</h1>

**rFortune** is a modern, cross-platform reimplementation of the classic Unix `fortune` program, written in Rust.

It displays a random quote or witty phrase from a plain text file, making it perfect for terminal startup messages,
scripting, or just a bit of inspiration.

![CI](https://github.com/umpire274/rfortune/actions/workflows/ci.yml/badge.svg)
[![License MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20Intel%20%7C%20macOS%20Apple%20Silicon-blue)](https://github.com/umpire274/rFortune/releases)
[![GitHub release](https://img.shields.io/github/v/release/umpire274/rfortune)](https://github.com/umpire274/rfortune/releases/latest)

---

### ✨ New in v0.5.5

**📚 Multiple fortune file support**

- Added support for **multiple fortune files** via the repeatable option `--file <PATH>`.  
  Example:
  ```bash
  rfortune --file ~/fortunes/dev --file ~/fortunes/humor
  ```
- Introduced the new configuration key `fortune_files` (list). When defined, it takes priority over `default_file`.
- **Automatic config migration**: if `fortune_files` is missing or empty, it is initialized automatically with the
  previous `default_file` value.
- **Unified JSON-based cache** shared across all configured fortune files.
- Intelligent **no-repeat mechanism**: rFortune now avoids showing the same quote twice in a row from the same file.

## 🧭 Improved cross-platform behavior

- Fixed an issue with the `app_dir()` function on Linux and macOS where `dirs::data_dir()` could return `None` in CI or
  headless environments.
- Added reliable fallbacks ensuring consistent paths:
    - macOS → `$HOME/Library/Application Support/rfortune`
    - Linux → `$HOME/.local/share/rfortune`
- Fixed inconsistent cache path resolution caused by incorrect `app_dir()` usage on Linux/macOS.
- Introduced the new helper function `ensure_cache_dir()` to centralize cache directory creation, improving reliability
  across `save_last_cache()` and `save_last_cache_json()`.
- Ensures predictable configuration and cache directory behavior across all systems, including GitHub Actions.

## ⚙️ Deprecated

- The old `files_fortune` key and single-file `print_random()` function have been removed or replaced by the new
  multi-file logic.

---

## 🚀 Features

- ✅ Cross-platform: works on **Linux**, **Windows**, **macOS Intel** and **Apple Silicon**
- ⚡ Fast and lightweight (native Rust binary)
- 📁 Simple input format: one or more lines per fortune, separated by `%`
- 🌹 UTF-8 support for multilingual content
- 🧩 Easily extensible
- 🧠 Built-in cache system to avoid showing the same fortune twice in a row
- ✨ New CLI with subcommands for config, file initialization and cache management

---

## 📦 Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/rfortune.svg)](https://repology.org/project/rfortune/versions)

### 🐧 AUR (Arch Linux)

[![AUR](https://img.shields.io/aur/version/rfortune)](https://aur.archlinux.org/packages/rfortune)

```bash
yay -S rfortune
# or
paru -S rfortune
```

### 🍺 Homebrew (macOS/Linux)

[![Homebrew](https://img.shields.io/badge/Homebrew-rFortune-orange.svg?logo=homebrew)](https://github.com/umpire274/homebrew-tap)

```bash
brew tap umpire274/tap
brew install rfortune
```

### 🦀 Crates.io (Rust)

[![Crates.io](https://img.shields.io/crates/v/rfortune)](https://crates.io/crates/rfortune)

```bash
cargo install rfortune
```

---

## 📥 Download

Precompiled binaries are available in the [Releases](https://github.com/umpire274/rfortune/releases) section.

| Platform                 | Architecture | File                                             |
|--------------------------|--------------|--------------------------------------------------|
| Windows                  | x86_64       | `rfortune-<version>-x86_64-pc-windows-msvc.zip`  |
| Linux                    | x86_64       | `rfortune-<version>-unknown-linux-gnu.tar.gz`    |
| macOS Intel Architecture | x86_64       | `rfortune-<version>-x86_64-apple-darwin.tar.gz`  |
| macOS Apple Silicon      | aarch64      | `rfortune-<version>-aarch64-apple-darwin.tar.gz` |

---

## 🔐 GPG Signature

All release archives are cryptographically signed with GPG.

- `.sig` files contain the ASCII-armored detached signature for the corresponding archive.
- You can verify the archive with:

```bash
gpg --verify rfortune-<version>-<target>.tar.gz.sig rfortune-<version>-<target>.tar.gz
```

---

## 🔑 Public Key

The releases are signed with the following GPG key:

* Key ID: 423FABCE0A1921FB
* Fingerprint: 8118 9716 9512 2A32 1F3D C04C 423F ABCE 0A19 21FB
* Download: https://github.com/umpire274.gpg

To import the key from a keyserver:

```sh
gpg --recv-keys 423FABCE0A1921FB
```

Or from OpenPGP server:

```sh
gpg --keyserver keys.openpgp.org --recv-keys 423FABCE0A1921FB
```

Then verify the fingerprint:

```sh
gpg --fingerprint 423FABCE0A1921FB
```

---

## 🚀 Usage

```sh
rfortune [OPTIONS]
rfortune <SUBCOMMAND>
```

Running `rfortune` without subcommands prints a random fortune from the default file (`rfortune.dat`).

---

## ⚙️ First-time setup

When `rfortune` is launched for the first time and no configuration directory exists,  
the application will ask whether to initialize its environment (creating the default  
configuration and fortune files).  
In non-interactive contexts, initialization happens automatically.

---

## 🧩 Options & Subcommands

| Command / Option             | Description                                                               |
|------------------------------|---------------------------------------------------------------------------|
| `-f`, `--file <PATH>`        | Use a custom fortune file instead of the default                          |
| `config init`                | Create the configuration file with default options                        |
| `config edit [--editor <E>]` | Open the configuration file in the system’s default or a specified editor |
| `file init`                  | Create a sample default fortune file (`rfortune.dat`)                     |
| `cache clear`                | Remove all cached last-used fortunes                                      |
| `-V`, `--version`            | Show version information                                                  |
| `-h`, `--help`               | Show help message                                                         |

---

## 💡 Examples

```bash
# Print a random fortune from the default file (rfortune.dat)
rfortune

# Print a random fortune from a specific file
rfortune --file ~/fortunes/misc

# Create the default configuration file in the user data directory
rfortune config init

# Open the configuration file in the system’s default text editor
rfortune config edit

# Open the configuration file with a specific editor (e.g. vi, nano, code)
rfortune config edit --editor vi

# Create a sample default fortune file (rfortune.dat)
rfortune file init

# Clear all cached last-used fortunes
rfortune cache clear
```

---

### Configuration (`rfortune.conf`)

Example:

```yaml
default_file: "/home/user/.local/share/rfortune/rfortune.dat"
print_title: true
use_cache: true

# Optional: load additional quote files
fortune_files:
  - "/usr/local/share/rfortune/philosophy.fort"
  - "/usr/local/share/rfortune/tech.fort"
```

Priority order:

1. `--file <PATH>` CLI argument(s)
2. `fortune_files` list in config
3. `default_file`

---

### Multiple Sources Configuration

You can load quotes from multiple files and rfortune will automatically
choose one at random:

```bash
rfortune --file my_quotes.fort --file jokes.fort --file tech.fort
```

Or configure them permanently:

```yaml
fortune_files:
  - "/path/to/my_quotes.fort"
  - "/path/to/jokes.fort"
```

If both are present, **CLI always wins**.

### Smart Quote Repetition Avoidance

rfortune keeps a small cache and automatically avoids repeating
the **same quote twice in a row**, but **only for quotes from the same file**.

This keeps the output natural across multiple sources.

---

### Migration from older versions

If your previous configuration did not contain `fortune_files`,
rfortune will automatically migrate your config by adding it and setting:

```yaml
fortune_files:
  - default_file
```

No manual action is required.

---

## 📁 Fortune File Format

Each fortune must be on one or more lines separated by `%`, like so:

  ```txt
  %
  The best way to get a good idea is to get a lot of ideas.
  %
  Do or do not. There is no try.
  %
  To iterate is human, to recurse divine.
  %
```

You may optionally add a title at the top of the file by starting the first line with #. The title will be printed
before the random quote:

```txt
# Murphy's Laws
%
Anything that can go wrong will go wrong.
%
If there's a possibility of several things going wrong, the one that will cause the most damage will be the one to go wrong.
%
```

---

## 🔒 License

This project is licensed under the MIT License.

© 2025 Alessandro Maestri

---

## 💡 Contributing

Pull requests are welcome! If you’d like to add support for more languages, improve performance, or fix bugs, feel free
to fork the repo and contribute.

---

## 🙌 Acknowledgments

Inspired by the classic BSD fortune program. Built with ❤️ in Rust.
