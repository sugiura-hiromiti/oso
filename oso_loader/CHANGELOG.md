# Changelog

All notable changes to the OSO Loader will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation comments throughout the codebase
- Module-level documentation for all major components
- Function-level documentation with examples and error conditions
- Inline comments explaining complex operations
- README.md with detailed usage instructions
- CHANGELOG.md for version tracking

### Documentation
- Added crate-level documentation explaining the bootloader's purpose and features
- Documented ELF parsing and loading process
- Explained architecture-specific kernel handoff procedures
- Added safety documentation for unsafe operations
- Documented UEFI integration and service wrappers

## [0.1.0] - Initial Release

### Added
- UEFI-based bootloader implementation
- ELF kernel loading support for multiple architectures
- Multi-architecture support (x86_64, aarch64, riscv64)
- UEFI services wrapper (chibi_uefi)
- Device tree support for ARM/RISC-V systems
- Graphics configuration for kernel handoff
- Memory management and allocation
- Architecture-specific kernel execution
- Error handling and recovery mechanisms

### Features
- Complete ELF file parsing and validation
- Program header and section header processing
- Dynamic linking information extraction
- Symbol table and relocation handling
- UEFI protocol interface wrappers
- Type-safe handle management
- Console operations for boot-time output
- File system access for kernel loading

### Architecture Support
- **x86_64**: System V AMD64 calling convention
- **AArch64**: MMU disabling, cache management, ARM calling convention
- **RISC-V 64**: Standard C calling convention, device tree support

### Safety
- Memory safety through Rust's ownership system
- Careful unsafe code isolation
- UEFI handle type safety
- Architecture-specific assembly validation
