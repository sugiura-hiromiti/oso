# Contributing to OSO

Thank you for your interest in contributing to OSO! This document provides guidelines and information for contributors to help maintain the project's quality and consistency.

## Project Overview

OSO is an experimental pure Rust operating system targeting aarch64 architecture, built with a macro-driven development philosophy. The project emphasizes:

- **Specification-driven development**: Generating code from official hardware specifications
- **Type safety in systems programming**: Leveraging Rust's advanced features for kernel development
- **Active reinvention**: Building from primary sources rather than copying existing implementations
- **High-level abstractions**: Using Rust's nightly features to achieve abstraction in low-level code

## Development Status

Understanding the current status of each component helps guide contributions:

- **oso_kernel**: Highly experimental, frequent breaking changes expected
- **oso_loader**: Almost stable, mature bootloader implementation
- **Procedural Macros**: Almost complete, with several components still work-in-progress
- **Development Utilities**: Current milestone focus - completing utility implementations

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust Toolchain**: Nightly Rust 1.90.0 or newer
- **QEMU**: Version 10 or newer for testing
- **System Tools**: 
  - `readelf` (part of binutils)
  - `mkfs.fat` for disk image creation
  - `hdiutil` (macOS) for disk mounting
- **Network Access**: Required for specification downloads

### Initial Setup

1. **Clone the Repository**
   ```bash
   git clone https://github.com/sugiura-hiromiti/oso.git
   cd oso
   ```

2. **Verify Build Environment**
   ```bash
   # Test the build system
   cargo xt
   
   # Verify cross-compilation targets
   rustup target add aarch64-unknown-none-elf
   rustup target add x86_64-unknown-none-elf
   ```

3. **Run Tests**
   ```bash
   # Run workspace tests
   cargo test --workspace
   
   # Run specific crate tests
   cargo test -p oso_proc_macro_logic
   ```

## Development Guidelines

### Code Style

#### Rust Conventions
- Follow standard Rust formatting with `rustfmt`
- Use `cargo clippy` for linting
- Prefer explicit types in public APIs
- Document all public interfaces with rustdoc comments

#### OSO-Specific Conventions
- **Macro-First Approach**: Prefer generating code from specifications over manual implementation
- **Primary Sources**: Always reference official specifications and documentation
- **Type Safety**: Leverage Rust's type system even in unsafe contexts
- **No Standard Library**: All code must work in `no_std` environments

### Architecture Principles

#### Specification Compliance
- Generate implementations from official specifications when possible
- Include references to specification sections in comments
- Validate generated code against real hardware/standards
- Maintain traceability from specification to implementation

#### Error Handling
- Use the `oso_error` crate for consistent error handling
- Provide context-rich error messages
- Include recovery suggestions where appropriate
- Handle both compile-time and runtime errors gracefully

#### Testing Strategy
- **Compile-time Testing**: Use procedural macros to generate validation tests
- **Cross-reference Testing**: Validate implementations against external tools
- **Specification Testing**: Ensure compliance with official standards
- **Integration Testing**: Test component interactions

### Contribution Areas

#### High Priority (Current Milestone)
- **Development Utilities**: Completing utility implementations
- **Build System**: Cross-platform support improvements
- **Documentation**: API documentation and examples
- **Testing Infrastructure**: Enhanced test coverage

#### Medium Priority
- **Kernel Features**: New device drivers and system calls
- **Loader Enhancements**: Additional architecture support
- **Macro Improvements**: New specification parsers
- **Performance**: Optimization and benchmarking

#### Low Priority
- **Cross-platform Build**: Non-macOS build support
- **User Experience**: End-user documentation and tutorials
- **Ecosystem Integration**: External tool compatibility

## Contribution Process

### Before Starting Work

1. **Check Existing Issues**: Look for related issues or discussions
2. **Create an Issue**: For significant changes, create an issue to discuss the approach
3. **Understand the Component**: Read the relevant crate's README and code
4. **Verify Dependencies**: Ensure you have required tools and network access

### Making Changes

#### For Kernel Development
- Understand the macro-driven architecture
- Reference ARM/x86 architecture manuals
- Test on both debug and release builds
- Verify cross-compilation compatibility

#### For Loader Development
- Follow UEFI specification compliance
- Test with multiple firmware implementations
- Ensure ELF compatibility across architectures
- Validate graphics output functionality

#### For Procedural Macros
- Test specification downloading and parsing
- Verify generated code correctness
- Include comprehensive error handling
- Document external dependencies clearly

#### For Development Tools
- Maintain cross-platform compatibility where possible
- Provide clear error messages and diagnostics
- Follow consistent CLI interface patterns
- Include comprehensive testing

### Testing Your Changes

#### Local Testing
```bash
# Build all components
cargo build --workspace

# Run comprehensive tests
cargo test --workspace

# Test with QEMU
cargo xt

# Test cross-architecture (if applicable)
cargo xt -86
```

#### Specification Validation
```bash
# For procedural macro changes
cargo test -p oso_proc_macro_logic

# For ELF-related changes
cargo test -p oso_loader
```

#### Integration Testing
```bash
# Full system test
cargo xt --release

# Debug mode testing
cargo xt --debug
```

### Submitting Changes

#### Pull Request Guidelines
1. **Clear Description**: Explain what changes were made and why
2. **Reference Issues**: Link to related issues or discussions
3. **Test Results**: Include test output and validation results
4. **Documentation**: Update relevant documentation
5. **Breaking Changes**: Clearly mark and explain breaking changes

#### Commit Message Format
```
component: brief description of change

Longer explanation of the change, including:
- Why the change was necessary
- What approach was taken
- Any trade-offs or limitations
- References to specifications or issues

Fixes #123
```

#### Review Process
- All changes require review before merging
- Reviewers will check for specification compliance
- Testing on multiple architectures may be required
- Documentation updates may be requested

## Specific Guidelines by Component

### oso_kernel
- **Status**: Highly experimental - breaking changes expected
- **Focus**: Macro-driven architecture and type safety
- **Testing**: Requires QEMU testing on target architectures
- **Documentation**: Document all unsafe code and hardware interactions

### oso_loader
- **Status**: Almost stable - maintain backward compatibility
- **Focus**: UEFI compliance and multi-architecture support
- **Testing**: Test with multiple UEFI firmware implementations
- **Documentation**: Maintain comprehensive API documentation

### Procedural Macros
- **Status**: Almost complete - focus on remaining WIP items
- **Focus**: Specification accuracy and error handling
- **Testing**: Validate against external tools and specifications
- **Documentation**: Document all external dependencies and requirements

### Development Utilities
- **Status**: Current milestone - active development area
- **Focus**: Code reuse and developer experience
- **Testing**: Cross-platform compatibility where possible
- **Documentation**: Provide usage examples and integration guides

## Communication

### Getting Help
- **Issues**: Use GitHub issues for bugs and feature requests
- **Discussions**: Use GitHub discussions for questions and ideas
- **Documentation**: Check crate-specific README files first

### Reporting Issues
When reporting issues, include:
- **Environment**: OS, Rust version, QEMU version
- **Component**: Which crate or component is affected
- **Reproduction**: Steps to reproduce the issue
- **Expected vs Actual**: What you expected vs what happened
- **Logs**: Relevant error messages or output

### Feature Requests
For feature requests, provide:
- **Use Case**: Why the feature is needed
- **Specification**: Reference to relevant standards if applicable
- **Implementation Ideas**: Suggested approach if you have one
- **Alternatives**: Other solutions you've considered

## Code of Conduct

### Principles
- **Respectful Communication**: Treat all contributors with respect
- **Constructive Feedback**: Provide helpful, actionable feedback
- **Learning Environment**: Support learning and knowledge sharing
- **Technical Focus**: Keep discussions focused on technical merit

### Standards
- Use welcoming and inclusive language
- Respect differing viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what's best for the project and community

## License

By contributing to OSO, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

## Recognition

Contributors are recognized through:
- Git commit history
- Release notes for significant contributions
- Documentation acknowledgments
- Community recognition for ongoing contributions

---

Thank you for contributing to OSO! Your efforts help advance the state of systems programming in Rust and contribute to the exploration of aarch64 development.
