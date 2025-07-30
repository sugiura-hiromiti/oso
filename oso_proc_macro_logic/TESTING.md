# Testing Guide for OSO Proc Macro Logic

This document describes the comprehensive test suite for the `oso_proc_macro_logic` crate.

## Test Structure

The test suite is organized into several categories:

### 1. Unit Tests (`src/*/tests`)

Each module contains its own unit tests that verify individual functions and components:

- **`lib.rs`**: Tests for the main crate functionality and `check_oso_kernel` function
- **`fonts_data.rs`**: Tests for font loading and bitmap conversion
- **`gen_wrapper_fn.rs`**: Tests for function argument extraction utilities
- **`impl_init.rs`**: Tests for trait implementation generation
- **`status_from_spec.rs`**: Tests for HTML parsing and UEFI status code extraction
- **`test_elf_header_parse.rs`**: Tests for ELF header parsing functionality
- **`test_program_headers_parse.rs`**: Tests for ELF program header parsing

### 2. Integration Tests (`tests/integration_tests.rs`)

Integration tests verify that different modules work together correctly:

- Cross-module functionality testing
- End-to-end workflow validation
- Dependency interaction verification
- Error propagation testing

### 3. Benchmark Tests (`benches/benchmarks.rs`)

Performance benchmarks to ensure the crate meets performance requirements:

- Font data processing performance
- HTML parsing performance
- String manipulation benchmarks
- Memory allocation patterns
- Error handling overhead

## Running Tests

### Quick Start

```bash
# Run all unit and integration tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests
```

### Using the Test Runner Script

The crate includes a comprehensive test runner script:

```bash
# Run all tests (unit + integration)
./run_tests.sh

# Run only unit tests
./run_tests.sh --unit-only

# Run only integration tests
./run_tests.sh --integration-only

# Run all tests including benchmarks
./run_tests.sh --all

# Run with verbose output
./run_tests.sh --verbose
```

### Benchmark Tests

```bash
# Run benchmark tests
cargo test --bench benchmarks

# Or using the test runner
./run_tests.sh --benchmarks
```

## Test Categories by Module

### Font Data Tests

- **File Loading**: Tests font file reading and parsing
- **Data Validation**: Ensures correct character count and format
- **Bitmap Conversion**: Tests conversion from text patterns to bitfields
- **Error Handling**: Tests behavior with invalid or missing files
- **Performance**: Benchmarks font processing speed

### Function Wrapper Tests

- **Argument Extraction**: Tests extraction of function arguments
- **Receiver Filtering**: Ensures `self` parameters are properly filtered
- **Complex Signatures**: Tests with generics, lifetimes, and patterns
- **Edge Cases**: Tests with empty signatures and various parameter types

### Implementation Generation Tests

- **Type Parsing**: Tests parsing of type lists from token streams
- **Code Generation**: Verifies generated trait implementations
- **Signed/Unsigned Handling**: Tests different behavior for signed vs unsigned types
- **Error Cases**: Tests handling of invalid type specifications

### HTML Parsing Tests

- **Element Search**: Tests finding elements by ID, class, and tag name
- **Table Parsing**: Tests extraction of data from HTML tables
- **Status Code Processing**: Tests conversion of HTML data to structured types
- **Malformed HTML**: Tests graceful handling of invalid HTML
- **Performance**: Benchmarks HTML parsing speed

### ELF Parsing Tests

- **Header Parsing**: Tests extraction of ELF header information
- **Program Headers**: Tests parsing of program header tables
- **Data Conversion**: Tests hex string to integer conversion
- **Field Processing**: Tests cleanup and normalization of parsed data
- **Error Handling**: Tests behavior when ELF files are missing or invalid

## Test Data and Fixtures

### Temporary Files

Many tests use temporary files to avoid dependencies on external files:

- Font data tests create temporary font files with known patterns
- ELF tests simulate readelf output where possible
- HTML tests use embedded HTML strings for parsing

### Mock Data

Tests include comprehensive mock data:

- Sample ELF header output
- UEFI specification HTML fragments
- Various font bitmap patterns
- Different function signatures for testing

## Test Coverage

The test suite aims for comprehensive coverage:

- **Line Coverage**: All major code paths are tested
- **Branch Coverage**: Different conditional branches are exercised
- **Error Coverage**: Error conditions and edge cases are tested
- **Integration Coverage**: Module interactions are verified

### Generating Coverage Reports

If you have `cargo-tarpaulin` installed:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# View the report
open coverage/tarpaulin-report.html
```

## Performance Testing

### Benchmark Categories

1. **Data Processing**: Font loading, HTML parsing, string manipulation
2. **Memory Usage**: Allocation patterns and memory efficiency
3. **Error Handling**: Overhead of error creation and propagation
4. **I/O Operations**: File reading and command execution

### Performance Expectations

The benchmarks include assertions for performance regressions:

- Font processing should complete within reasonable time limits
- HTML parsing should handle moderately large documents efficiently
- String operations should be optimized for the common use cases
- Memory allocations should be minimal and efficient

## Continuous Integration

The test suite is designed to work in CI/CD environments:

- Tests avoid dependencies on external files where possible
- Environment-specific tests are properly marked with `#[ignore]`
- Performance tests have reasonable thresholds
- Error messages are clear and actionable

### CI Configuration Example

```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: ./run_tests.sh --all
```

## Debugging Tests

### Common Issues

1. **Missing ELF File**: Some tests require `target/oso_kernel.elf`
   - These tests are marked with `#[ignore]` by default
   - Run with `cargo test -- --ignored` if the file exists

2. **Environment Dependencies**: Some tests depend on system tools
   - `readelf` command must be available for ELF parsing tests
   - Tests gracefully handle missing dependencies

3. **Temporary File Issues**: Tests use temporary files extensively
   - Ensure `/tmp` is writable
   - Tests clean up temporary files automatically

### Debugging Tips

```bash
# Run a specific test with output
cargo test test_name -- --nocapture

# Run tests in single-threaded mode (helpful for debugging)
cargo test -- --test-threads=1

# Show all test output
cargo test -- --nocapture --test-threads=1
```

## Adding New Tests

### Guidelines

1. **Unit Tests**: Add to the appropriate module's `tests` submodule
2. **Integration Tests**: Add to `tests/integration_tests.rs`
3. **Benchmarks**: Add to `benches/benchmarks.rs`
4. **Documentation**: Update this file when adding new test categories

### Test Naming Conventions

- Unit tests: `test_function_name_scenario`
- Integration tests: `test_module_integration`
- Benchmarks: `benchmark_operation_name`
- Error tests: `test_function_name_error_case`

### Example Test Structure

```rust
#[test]
fn test_function_name_success_case() {
    // Arrange
    let input = create_test_input();
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}

#[test]
#[should_panic(expected = "specific error message")]
fn test_function_name_error_case() {
    let invalid_input = create_invalid_input();
    function_under_test(invalid_input);
}
```

## Test Maintenance

### Regular Tasks

1. **Update Test Data**: Keep mock data current with real-world examples
2. **Performance Baselines**: Update benchmark thresholds as needed
3. **Dependency Updates**: Ensure tests work with updated dependencies
4. **Coverage Analysis**: Regularly check and improve test coverage

### Refactoring Tests

When refactoring code:

1. Update corresponding tests
2. Ensure test names remain descriptive
3. Maintain test isolation
4. Update integration tests for API changes

## Conclusion

This comprehensive test suite ensures the reliability, performance, and correctness of the `oso_proc_macro_logic` crate. The combination of unit tests, integration tests, and benchmarks provides confidence in the crate's functionality across different use cases and environments.

For questions or issues with the test suite, please refer to the main project documentation or open an issue in the project repository.
