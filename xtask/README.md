# xtask

A build and run utility for the OSO project that automates the process of building, packaging, and running the OSO kernel and loader in QEMU.

## Overview

The `xtask` crate is a build tool that:

1. Builds the OSO loader (UEFI application) and kernel
2. Creates and formats a disk image
3. Mounts the disk image and copies the built artifacts
4. Configures and runs QEMU with the appropriate firmware and disk image

## Features

- Cross-architecture support (aarch64, x86_64, riscv64[^1])
- Debug mode with GDB support
- Automatic OVMF firmware management
- Disk image creation and mounting
- Configurable build modes (debug/release)

[^1]: riscv64 support is experimental

## Usage

Run from the OSO project root:

```bash
cargo run -p xtask [OPTIONS]

# short hand
cargo xt
```

### Options

- `-r`, `--release`: Build in release mode (default is debug mode)
- `-86`, `-x86_64`: Build for x86_64 architecture (default is aarch64)
- `--debug`: Enable debug mode with GDB support (listens on port 12345)

## Architecture

The crate is organized into several modules:

- `main.rs`: Entry point that creates a Builder and runs the build and run process
- `builder.rs`: Core functionality for building the loader and kernel, creating disk images, and running QEMU
- `qemu.rs`: QEMU configuration and firmware management
- `shell.rs`: Command-line argument parsing and architecture definitions
- `workspace.rs`: Workspace management and path resolution

## Dependencies

- `anyhow`: Error handling
- `colored`: Terminal output coloring
- `fs-err`: File system operations with improved error messages
- `ovmf-prebuilt`: OVMF firmware management
- `serde_json`: JSON parsing for target configuration
- `toml`: TOML parsing for Cargo.toml files
- `util_common_code`: Common utilities from the OSO project

## System Requirements

- QEMU
- macOS (for hdiutil commands)
- mkfs.fat
- Rust toolchain with cross-compilation targets

## Build Process

1. The builder detects the OSO workspace structure
2. Builds the loader as a UEFI application for the target architecture
3. Builds the kernel for the target architecture
4. Creates and formats a FAT32 disk image
5. Mounts the disk image
6. Creates the EFI boot directory structure
7. Copies the loader and kernel to the appropriate locations
8. Unmounts the disk image
9. Runs QEMU with the appropriate configuration

## Debugging

When run with the `--debug` flag, QEMU will start with GDB server enabled on port 12345 and wait for a debugger to connect before starting execution.

## Cleanup

The builder automatically cleans up temporary files and unmounts disk images when it's done or if an error occurs.
