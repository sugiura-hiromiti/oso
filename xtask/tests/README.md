# xtask Test Suite

This directory contains comprehensive tests for the xtask crate, covering all major functionality and edge cases.

## Test Structure

### Integration Tests (`integration_tests.rs`)
- Tests end-to-end functionality of the xtask binary
- Verifies workspace detection and structure
- Tests that the binary can be built successfully
- Validates workspace directory structure and manifest files

### Builder Tests (`builder_tests.rs`)
- Tests the core Builder functionality
- Verifies path generation for disk images and mount points
- Tests EFI boot directory structure creation
- Validates build artifact path construction
- Tests cleanup operations and disk image handling

### QEMU Tests (`qemu_tests.rs`)
- Tests QEMU configuration and command-line argument generation
- Verifies firmware management and OVMF integration
- Tests architecture-specific QEMU arguments
- Validates persistent flash memory configuration
- Tests block device and boot menu setup

### Workspace Tests (`workspace_tests.rs`)
- Tests workspace detection and management
- Verifies TOML file parsing and crate information extraction
- Tests architecture target tuple generation
- Validates JSON file loading and build artifact detection
- Tests environment variable handling

### Shell Extended Tests (`shell_extended_tests.rs`)
- Extends the existing shell module tests
- Tests command-line argument parsing logic
- Verifies feature string parsing and categorization
- Tests architecture validation and boot file patterns
- Validates option precedence and flag recognition

### Error Handling Tests (`error_handling_tests.rs`)
- Tests error conditions and edge cases
- Verifies graceful handling of missing files and directories
- Tests malformed configuration file handling
- Validates error messages and recovery mechanisms
- Tests resource limit and permission error handling

### Test Runner (`test_runner.rs`)
- Provides utilities for setting up test environments
- Creates mock workspace structures and build artifacts
- Offers helper functions for test configuration
- Includes utilities for checking system requirements

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test Modules
```bash
# Integration tests
cargo test --test integration_tests

# Builder tests
cargo test --test builder_tests

# QEMU tests
cargo test --test qemu_tests

# Workspace tests
cargo test --test workspace_tests

# Shell tests
cargo test --test shell_extended_tests

# Error handling tests
cargo test --test error_handling_tests

# Test runner utilities
cargo test --test test_runner
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests in Release Mode
```bash
cargo test --release
```

## Test Coverage

The test suite covers:

### Core Functionality
- ✅ Workspace detection and initialization
- ✅ Build process orchestration
- ✅ QEMU configuration and execution
- ✅ Disk image creation and mounting
- ✅ Firmware management
- ✅ Command-line argument parsing

### Architecture Support
- ✅ aarch64 (ARM 64-bit)
- ✅ x86_64 (Intel/AMD 64-bit)
- ✅ riscv64 (RISC-V 64-bit) - basic support

### Build Modes
- ✅ Debug builds
- ✅ Release builds
- ✅ Feature flag handling

### Error Conditions
- ✅ Missing workspace directories
- ✅ Malformed configuration files
- ✅ Invalid command-line arguments
- ✅ Missing build artifacts
- ✅ File permission errors
- ✅ Network connectivity issues (simulated)

### System Integration
- ✅ Environment variable handling
- ✅ External command execution
- ✅ File system operations
- ✅ Temporary file management

## Test Dependencies

The tests use the following additional dependencies:

- `tempfile`: For creating temporary directories and files
- `assert_cmd`: For testing command-line applications
- `predicates`: For advanced assertion predicates

## Mock Data and Fixtures

Tests use mock data to avoid dependencies on external systems:

- Mock workspace structures with minimal Cargo.toml files
- Mock build artifacts (EFI files, kernel binaries)
- Mock target JSON files with linker configurations
- Mock firmware files for OVMF testing
- Mock disk images for testing disk operations

## System Requirements for Tests

### Required Tools (for full test coverage)
- Rust toolchain with cargo
- Basic Unix utilities (ls, which, etc.)

### Optional Tools (for extended testing)
- QEMU (for QEMU-related tests)
- Build tools (for integration tests)

### Platform Support
- ✅ macOS (primary development platform)
- ✅ Linux (should work with minor adjustments)
- ⚠️ Windows (may require path separator adjustments)

## Test Isolation

Tests are designed to be isolated and can run in parallel:

- Each test uses its own temporary directory
- No shared global state between tests
- Mock data prevents external dependencies
- Cleanup is performed automatically via `TempDir`

## Continuous Integration

The test suite is designed to work in CI environments:

- No external network dependencies (except for optional OVMF downloads)
- Reasonable resource requirements
- Clear pass/fail criteria
- Detailed error messages for debugging

## Adding New Tests

When adding new functionality to xtask, please:

1. Add unit tests for individual functions
2. Add integration tests for end-to-end workflows
3. Add error handling tests for failure cases
4. Update this documentation
5. Ensure tests are isolated and deterministic

### Test Naming Conventions

- `test_<functionality>` for basic functionality tests
- `test_<functionality>_error` for error condition tests
- `test_<functionality>_edge_case` for edge cases
- `test_mock_<component>` for mock/simulation tests

### Test Organization

- Group related tests in the same file
- Use descriptive test names
- Include docstrings for complex test logic
- Use helper functions to reduce code duplication

## Debugging Tests

### Running Individual Tests
```bash
cargo test test_specific_function_name
```

### Running Tests with Debug Output
```bash
RUST_LOG=debug cargo test -- --nocapture
```

### Running Tests with Backtraces
```bash
RUST_BACKTRACE=1 cargo test
```

## Performance Considerations

The test suite is designed to be fast:

- Most tests use in-memory operations
- File system operations use temporary directories
- Mock data is small and efficient
- Tests avoid unnecessary external command execution

Typical test run time: < 30 seconds on modern hardware.

## Known Limitations

- Some tests require Unix-like systems (marked with `#[cfg(unix)]`)
- QEMU tests are optional and skip if QEMU is not installed
- Network-related tests use simulation rather than real network calls
- Some file permission tests may behave differently on different filesystems

## Contributing

When contributing tests:

1. Follow the existing test structure and naming conventions
2. Ensure tests are deterministic and don't depend on external state
3. Add appropriate error handling and cleanup
4. Update documentation for new test categories
5. Consider both positive and negative test cases
