# oso_kernel

The core kernel implementation for the OSO operating system, featuring a macro-driven architecture that maximizes code generation and testing coverage while maintaining high-level abstractions.

## Status

**Highly Experimental** - This kernel is under active development with frequent breaking changes.

## Overview

The `oso_kernel` is designed around the philosophy of macro-driven development, where critical system components are automatically generated from specifications to ensure correctness and reduce redundant code. The kernel leverages Rust's advanced type system and nightly features to achieve higher levels of abstraction typically not seen in systems programming.

## Architecture

### Module Structure

```
oso_kernel/src/
├── lib.rs              # Kernel library entry point
├── main.rs             # Kernel main entry point
├── base.rs             # Base kernel functionality
├── app.rs              # Application execution system
├── driver.rs           # Device driver interface
├── app/                # Application subsystem modules
├── base/               # Core kernel utilities and abstractions
└── driver/             # Hardware device drivers
```

### Core Modules

#### `app` - Application Execution System
Handles user-space application loading, execution, and management. Provides the interface between kernel services and user applications.

#### `base` - Basic Kernel Library
Contains fundamental kernel data structures, memory management primitives, and core abstractions that other modules depend on.

#### `driver` - Device Control
Hardware abstraction layer providing unified interfaces to various hardware components including graphics, storage, and input devices.

## Features

### Macro-Driven Design
- **Specification-based Generation**: Critical kernel structures and interfaces are generated from hardware specifications
- **Test Coverage**: Macros ensure comprehensive testing of generated code
- **Code Deduplication**: Eliminates redundant implementations through intelligent code generation

### Advanced Rust Features
The kernel extensively uses nightly Rust features to achieve:
- Higher-level abstractions in systems code
- Compile-time guarantees for system correctness
- Zero-cost abstractions for performance-critical paths

### Graphics Support
Configurable pixel format support through Cargo features:
- `rgb`: RGB pixel format
- `bgr`: BGR pixel format  
- `bitmask`: Bitmask pixel format
- `bltonly`: Block transfer only mode (default)

## Dependencies

```toml
[dependencies]
oso_error = { path = "../oso_error" }           # Error handling system
oso_no_std_shared = { path = "../oso_no_std_shared" }  # Shared no_std utilities
oso_proc_macro = { path = "../oso_proc_macro" }        # Macro-driven code generation
```

## Target Architectures

### Primary Target
- **aarch64**: Full support with custom target specification (`aarch64-unknown-none-elf.json`)

### Secondary Target  
- **x86_64**: Partial support with custom target specification (`x86_64-unknown-none-elf.json`)

## Build Configuration

The kernel uses custom target specifications for bare-metal execution:

### AArch64 Configuration
```json
{
  "arch": "aarch64",
  "data-layout": "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
  "llvm-target": "aarch64-unknown-none",
  "target-pointer-width": "64"
}
```

### Build Requirements
- Rust nightly toolchain (1.90.0+)
- Custom target support
- Cross-compilation capabilities

## Development Philosophy

### Macro-First Approach
Rather than writing repetitive low-level code, the kernel generates implementations from specifications:
- Hardware register definitions from official documentation
- Protocol implementations from standards
- Test cases from specification requirements

### Type Safety in Systems Code
Leverages Rust's ownership system and type safety even in kernel space:
- Memory safety without garbage collection
- Compile-time verification of system invariants
- Zero-cost abstractions for performance

### Specification Compliance
All implementations are derived from primary sources:
- Hardware vendor specifications
- Industry standards and protocols
- Official architecture references

## Usage

The kernel is designed to be loaded by the `oso_loader` UEFI bootloader:

```bash
# Build the kernel (from project root)
cargo build -p oso_kernel --target aarch64-unknown-none-elf

# Or use the xtask build system
cargo xt
```

## Integration

### With OSO Loader
The kernel expects to be loaded as an ELF binary named `oso_kernel.elf` by the OSO loader. The loader provides:
- Memory layout information
- Graphics configuration
- Device tree data (on ARM systems)
- System initialization state

### Runtime Environment
- **No Standard Library**: Operates in `no_std` environment
- **Bare Metal**: Direct hardware access without OS layer
- **Custom Allocator**: Implements its own memory management
- **Interrupt Handling**: Direct hardware interrupt management

## Current Limitations

As an experimental kernel, current limitations include:
- Limited device driver support
- Basic memory management
- Minimal user-space interface
- Architecture-specific features may be incomplete

## Contributing

See the main project [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

## License

MIT OR Apache-2.0
