# 🥠 rFortune

**rFortune** is a modern, cross-platform reimplementation of the classic Unix `fortune` program, written in Rust.

It displays a random quote or witty phrase from a plain text file, making it perfect for terminal startup messages,
scripting, or just a bit of inspiration.

![CI](https://github.com/umpire274/rfortune/actions/workflows/ci.yml/badge.svg)
[![Licenza MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS%20Intel%20%7C%20macOS%20Apple%20Silicon-blue)](https://github.com/umpire274/rFortune/releases)
[![Versione](https://img.shields.io/badge/version-0.3.0-orange)](https://github.com/umpire274/rfortune/releases/tag/v0.3.0)

---

## 🚀 Features

- ✅ Cross-platform: works on **Linux**, **Windows**, **macOS Intel** and **Apple Silicon**
- ⚡ Fast and lightweight (native Rust binary)
- 📁 Simple input format: one or more lines per fortune, separated by `%`
- 🌹 UTF-8 support for multilingual content
- 🧩 Easily extensible
- 🧠 Built-in cache system to avoid showing the same fortune twice in a row
- ✨ Supports various command-line options including file selection, cache management, version display, and more

---

## 📦 Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/rfortune.svg)](https://repology.org/project/rfortune/versions)

### 🐧 AUR (Arch Linux)
[![AUR](https://img.shields.io/aur/version/rfortune)](https://aur.archlinux.org/packages/rfortune)

```bash
yay -S rfortune
# oppure
paru -S rfortune
```

### 🍺 Homebrew (macOS/Linux)
[![Homebrew Tap](https://img.shields.io/badge/homebrew-tap-brightgreen)](https://github.com/umpire274/homebrew-rfortune)

```bash
brew tap umpire274/rfortune
brew install rfortune
```

### 🪟 Scoop (Windows)
[![Scoop](https://img.shields.io/badge/scoop-rfortune-blue)](https://github.com/ScoopInstaller/Main/pull/XXX)

```powershell
scoop bucket add main
scoop install rfortune
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

## 🔐 Initialization (optional)

To create the default fortune directory and a starter `rfortunes.dat` file:

```bash
rfortune --init
```

- On **Linux/macOS**: creates `/usr/local/share/rfortune/rfortunes.dat`
- On **Windows**: creates `%APPDATA%\rfortune\rfortunes.dat`

---

## 🚀 Usage

```sh
rfortune [--file path/to/quotes.dat]
```

---

## 🧩 Options

| Option            | Description                                |
|-------------------|--------------------------------------------|
| `-f`, `--file`    | Use a custom file of fortunes              |
| `--init`          | Create the default directory and test file |
| `--clear-cache`   | Delete all cached quote history            |
| `-V`, `--version` | Show version                               |
| `-h`, `--help`    | Show help message                          |

> If no file is specified, the program defaults to platform-specific location.

---

## 🧪 Example

```sh
$ rfortune
Never trust a computer you can't throw out a window. — Steve Wozniak
```

---

## 📁 File Format

Each fortune must be on one or more lines separated by '%', like so:

```txt
%
The best way to get a good idea is to get a lot of ideas.
%
Do or do not. There is no try.
%
To iterate is human, to recurse divine.
%
```

You may optionally add a title at the top of the file by starting the first line with #. The title will be printed before the random quote:

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

