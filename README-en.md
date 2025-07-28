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

## Philosophy & Characteristics

- [x] AArch64-first design
- [x] Pure Rust implementation
- [x] No external dependencies
  - The xtask crate (for developer utilities) may use dependencies
  - Some use of external crates is permitted when aligned with curiosity-driven learning, e.g.:
    - Web scraping for automatically generating implementations from specs via proc macros
- [x] Standards compliant
  - Strives to follow de facto standards, avoiding unnecessary “original formats”
- [x] Intentional Reinvention
  - Instead of mimicking existing implementations, all code is written from scratch based on primary sources (specs, technical references)
  - Aims to reinterpret what an OS can be — and what Rust can express at the OS level
- [x] Advanced Rust usage
  - Actively explores underused features and possibilities of Rust in OS development

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
