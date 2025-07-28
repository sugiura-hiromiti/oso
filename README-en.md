[日本語版](README.md)

> This README contains AI-generated content that has been manually refined
> Last updated: 250728

# `oso` — Experimental Pure Rust OS for aarch64

**`oso`** is a completely self-made OS (except for external tools like QEMU and the Rust language itself) that pursues direct hardware control at the low level while maximizing Rust's type safety and abstraction capabilities.
From a custom UEFI bootloader to macro-driven kernel design, the goal is to pursue abstraction even in low-level development.
Additionally, it primarily targets aarch64, which still has limited online resources.
This is for the following reasons:

- To serve as a pioneer and reference material in a minor but high-potential field
- To train the developer's (my) self-reliance abilities

## QuickStart

Please ensure that the [required tools](#build) are installed before running commands

```bash
git clone https://github.com/sugiura-hiromiti/oso.git
cd oso
cargo xt
```

## Development Philosophy & Features

- [x] aarch64 targeted
- [x] pure Rust
- [x] no dependencies
  - External crates are used in the development auxiliary crate `xtask`
  - Additionally, external crates are used for the following purposes to prioritize my technical curiosity:
    - Web scraping: Automatically generating implementations with proc macros based on specifications
- [x] Standards compliant
  - Developed to respect de facto standards and minimize custom specifications
- [x] Active reinvention
  - Code is built from scratch based on primary sources (specifications and references) without copying existing implementations
  - This is to reinterpret the role and possibilities of OS during development and observe what OS and Rust can do from a level playing field
- [x] Active use of Rust's higher-order features
  - To explore Rust-specific advantages in OS development that existing projects have overlooked

## Project Structure

This repository is a [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) composed of multiple crates:

### `oso_kernel`

The crate that constitutes the kernel body

**Features**

- Extensive use of nightly Rust features
- Module structure:
  - `app`: Application execution system
  - `base`: Basic library
  - `driver`: Device control
- Integration with other crates:
  - `oso_no_std_shared`: Shared library for no_std environment
  - `oso_proc_macro`: Automatic code generation through macros
  - `oso_error`: Error handling system

### `oso_loader`

Custom UEFI-compatible bootloader implementation.

- ELF format kernel loading functionality
- Graphics support (RGB/BGR/Bitmask formats)
- Used crates:
  - `oso_no_std_shared`
  - `oso_proc_macro`
  - `oso_error`

### `xtask`

Developer auxiliary tool suite.

- Build assistance for QEMU and UEFI targets
- Startup scripts
- Deployment and test automation processes

### Auxiliary Crates List

| Crate Name             | Description                                                                    |
| ---------------------- | ------------------------------------------------------------------------------ |
| `oso_no_std_shared`    | Provides basic data structures and utilities shared in no_std environment     |
| `oso_proc_macro_logic` | Internal logic implementation and testing for procedural macros               |
| `oso_proc_macro`       | Macro suite supporting kernel struct, parser, and test generation             |
| `oso_error`            | Common error types and error handling logic                                   |
| `util_common_code`     | General-purpose code shared between development tools                         |

## build

Requirements:

- Rust (nightly)
- QEMU
- macOS (currently required due to use of hdiutil; multi-platform support is planned)

```bash
# Build all crates, mount binaries, and launch in QEMU
cargo xt

# Partial support for x86_64 as well
cargo xt -86
```
