# OSO Loader

A UEFI-based bootloader for the OSO operating system that handles ELF kernel loading and system initialization across multiple architectures.

## Overview

The OSO Loader is a lightweight, efficient bootloader designed to load ELF format kernels in UEFI environments. It provides essential bootloader functionality while maintaining simplicity and reliability.

## Features

### Core Functionality
- **ELF Kernel Loading**: Complete ELF file parsing and loading with support for all standard sections
- **Multi-architecture Support**: Native support for x86_64, aarch64, and riscv64 architectures
- **UEFI Integration**: Comprehensive UEFI services wrapper with type-safe interfaces
- **Memory Management**: Intelligent memory allocation and page management
- **Device Tree Support**: Hardware configuration through device tree for ARM/RISC-V systems

### Architecture-Specific Features
- **AArch64**: MMU disabling, cache management, and ARM calling conventions
- **x86_64**: System V AMD64 calling convention support
- **RISC-V 64**: Standard C calling convention support

### Graphics and I/O
- **Graphics Configuration**: Frame buffer setup for kernel graphics operations
- **Console Operations**: Text input/output during boot process
- **File System Access**: ELF kernel loading from UEFI file systems

## Architecture

### Module Structure

```
oso_loader/
├── src/
│   ├── lib.rs              # Main library with core functionality
│   ├── main.rs             # UEFI application entry point
│   ├── load.rs             # Kernel and graphics loading
│   ├── elf.rs              # ELF file parsing and structures
│   ├── chibi_uefi.rs       # Lightweight UEFI wrapper
│   ├── chibi_uefi/         # UEFI service modules
│   └── raw/                # Raw UEFI types and protocols
└── Cargo.toml
```

### Key Components

#### 1. ELF Parser (`elf.rs`)
- Complete ELF format support (32-bit and 64-bit)
- Program header and section header processing
- Symbol table and relocation handling
- Dynamic linking information extraction

#### 2. Kernel Loader (`load.rs`)
- ELF file system access
- Memory allocation for kernel segments
- Segment copying and .bss initialization
- Graphics configuration setup

#### 3. UEFI Wrapper (`chibi_uefi/`)
- Type-safe UEFI handle management
- Boot and runtime service abstractions
- Protocol interface wrappers
- Memory and device management

#### 4. Architecture Support
- Architecture-specific kernel handoff
- MMU and cache management (AArch64)
- Calling convention handling
- Device tree integration

## Boot Process

The OSO Loader follows a structured boot sequence:

1. **UEFI Initialization**
   - System table setup
   - Device connection and enumeration
   - Console initialization

2. **Kernel Loading**
   - ELF file location and reading
   - Format validation and parsing
   - Memory allocation at required addresses
   - Segment loading and initialization

3. **System Preparation**
   - Device tree retrieval
   - Graphics configuration
   - Memory map preparation

4. **Boot Services Exit**
   - UEFI boot services termination
   - Transition to runtime environment

5. **Kernel Handoff**
   - Architecture-specific preparation
   - MMU disabling (AArch64)
   - Cache management
   - Control transfer to kernel entry point

## Configuration

### Cargo Features

The loader supports several compile-time features:

- `rgb`: RGB pixel format support
- `bgr`: BGR pixel format support  
- `bitmask`: Bitmask pixel format support
- `bltonly`: Block transfer only mode (default)

### Build Configuration

```toml
[dependencies]
oso_error = { path = "../oso_error" }
oso_no_std_shared = { path = "../oso_no_std_shared" }
oso_proc_macro = { path = "../oso_proc_macro" }
```

## Usage

### Building

```bash
cargo build --target x86_64-unknown-uefi
# or
cargo build --target aarch64-unknown-uefi
# or  
cargo build --target riscv64gc-unknown-none-elf
```

### Deployment

1. Build the loader for your target architecture
2. Copy the resulting `.efi` file to your UEFI system partition
3. Place your kernel as `oso_kernel.elf` in the same directory
4. Boot from UEFI firmware

### Kernel Requirements

Your kernel must:
- Be in valid ELF format
- Be named `oso_kernel.elf`
- Have appropriate entry point for your architecture
- Accept device tree address as first parameter

## Architecture-Specific Details

### AArch64
- Disables MMU before kernel handoff
- Performs instruction cache invalidation
- Uses ARM calling convention
- Supports device tree configuration

### x86_64
- Uses System V AMD64 calling convention
- Supports UEFI graphics output
- Standard x86_64 memory layout

### RISC-V 64
- Uses standard C calling convention
- Device tree support for hardware configuration
- RISC-V specific memory management

## Error Handling

The loader uses a comprehensive error handling system:

- **Parse Errors**: ELF format validation and parsing errors
- **UEFI Errors**: System service and protocol errors  
- **Memory Errors**: Allocation and management failures
- **I/O Errors**: File system and device access errors

Critical errors result in panic with diagnostic information, while recoverable errors are propagated through the `Rslt` type system.

## Safety Considerations

The loader operates in a privileged UEFI environment and includes:

- Memory safety through Rust's ownership system
- Careful unsafe code isolation and documentation
- UEFI handle type safety
- Architecture-specific assembly validation

## Dependencies

- `oso_error`: Error handling and result types
- `oso_no_std_shared`: Shared no_std utilities
- `oso_proc_macro`: Compile-time code generation

## Contributing

When contributing to the OSO Loader:

1. Maintain comprehensive documentation
2. Follow Rust safety guidelines
3. Test on target architectures
4. Update documentation for new features
5. Consider UEFI specification compliance

## License

[License information would go here]

## Technical References

- [UEFI Specification](https://uefi.org/specifications)
- [ELF Format Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
- [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest)
- [RISC-V Specification](https://riscv.org/technical/specifications/)
