[日本語版](README.md)

> This README contains AI-generated content that has been manually refined
> Last updated: 250728

# `oso` — An Experimental Pure Rust OS for AArch64

**`oso`** is a fully handcrafted operating system written in Rust, targeting the AArch64 architecture. It aims to leverage Rust's type safety and abstraction capabilities while pursuing low-level, direct hardware control — with no external dependencies except for fundamental tools like QEMU and the Rust toolchain itself.

From its custom UEFI bootloader to a macro-driven kernel design, `oso` is an attempt to bring abstract reasoning to bare-metal development.

### Why AArch64?

AArch64 is chosen as the primary target for the following reasons:

- Though still underrepresented in online resources, it holds high potential — `oso` aims to be both a pioneer and a reference in this space.
- It pushes the developer (myself) to cultivate independent problem-solving skills, relying not on blog posts, but on raw specification and self-reasoning.

---

## 🔧 Quick Start

Before running the commands below, make sure you've [installed the necessary tools](#build).

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
