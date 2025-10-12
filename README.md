# 🥠 rFortune

**rFortune** is a modern, cross-platform reimplementation of the classic Unix `fortune` program, written in Rust.

It displays a random quote or witty phrase from a plain text file, making it perfect for terminal startup messages,
scripting, or just a bit of inspiration.

![CI](https://github.com/umpire274/rfortune/actions/workflows/ci.yml/badge.svg)
[![License MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20Intel%20%7C%20macOS%20Apple%20Silicon-blue)](https://github.com/umpire274/rFortune/releases)
[![GitHub release](https://img.shields.io/github/v/release/umpire274/rfortune)](https://github.com/umpire274/rfortune/releases/latest)


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
brew tap umpire274/rfortune
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

## 🧩 Options & Subcommands

| Command / Option      | Description                                           |
|-----------------------|-------------------------------------------------------|
| `-f`, `--file <PATH>` | Use a custom fortune file instead of the default      |
| `config init`         | Create the configuration file with default options    |
| `file init`           | Create a sample default fortune file (`rfortune.dat`) |
| `cache clear`         | Remove all cached last-used fortunes                  |
| `-V`, `--version`     | Show version information                              |
| `-h`, `--help`        | Show help message                                     |

---

## 🧪 Examples

```sh
# Print a random fortune from the default file
rfortune

# Print a random fortune from a specific file
rfortune --file ~/fortunes/misc

# Initialize configuration file
rfortune config init

# Create a sample default fortune file
rfortune file init

# Clear all cached last-used fortunes
rfortune cache clear
```

---

## 📁 File Format

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
