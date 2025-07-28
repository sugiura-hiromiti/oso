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

### oso_kernel

The core kernel crate.

Highlights:

- Uses many advanced nightly Rust features
- Modular structure:
  - app: Application runtime layer
  - base: Core abstractions and utilities
  - driver: Device drivers
  - gui: Graphical user interface
- Integrates with:
  - oso_bridge: Interface between bootloader and kernel
  - oso_binary_parser: Parses binary formats like DeviceTree (.dtb)
  - oso_proc_macro: Procedural macros for code generation
  - oso_error: Unified error handling framework

### oso_loader

A custom UEFI-compliant bootloader.

- Includes chibi_uefi: a minimalist UEFI wrapper written in Rust
- Supports ELF-based kernel loading
- Uses:
  - oso_bridge
  - oso_error

### xtask

Developer-focused CLI utilities.

- Builds for UEFI/QEMU environments
- Handles boot image creation and QEMU execution
- Automates deploy/test/dev flow

### Supporting Crates

| クレート名             | 説明                                                                         |
| ---------------------- | ---------------------------------------------------------------------------- |
| `oso_binary_parser`    | Provides a general framework for parsing ELF, DeviceTree, and other binaries |
| `oso_proc_macro_logic` | Internal logic and tests for procedural macro expansion                      |
| `oso_proc_macro`       | Macros for generating kernel structs, parsers, and test scaffolding          |
| `oso_error`            | Common error types and handling logic                                        |
| `oso_bridge`           | Shared interface structures between bootloader and kernel                    |

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
