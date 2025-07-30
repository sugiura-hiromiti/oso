# CI/CD Setup for oso

This document explains the CI/CD configuration for the oso OS project.

## Workflows

### 1. Main CI Pipeline (`.github/workflows/ci.yml`)

Runs on every push and pull request to `main` and `develop` branches.

**Jobs:**
- **Check and Lint**: Code formatting, clippy linting, and documentation checks
- **Build aarch64**: Builds kernel and loader for aarch64 target
- **Test**: Runs unit tests for std-compatible crates
- **Integration**: Tests the build process with QEMU (dry-run)
- **Security**: Runs cargo audit for security vulnerabilities
- **Dependency Check**: Verifies no_std compliance for kernel/loader

**Key Features:**
- Focuses on aarch64 architecture (as per project requirements)
- Uses nightly Rust with proper target installation
- Caches cargo registry for faster builds
- Tests only crates that can run in std environment

### 2. Release Pipeline (`.github/workflows/release.yml`)

Triggered on git tags starting with `v*` (e.g., `v1.0.0`).

**Features:**
- Builds release artifacts for aarch64
- Creates GitHub releases with binaries
- Packages kernel, loader, and xtask binaries

### 3. Documentation Pipeline (`.github/workflows/docs.yml`)

Builds and deploys documentation to GitHub Pages.

**Features:**
- Generates rustdoc for all workspace crates
- Deploys to GitHub Pages automatically
- Includes private items documentation

## Project-Specific Considerations

### aarch64 Focus
- All builds target custom `aarch64-unknown-none-elf.json` for kernel
- QEMU aarch64 system emulation for testing
- No x86_64 builds (as requested)

### no_std Environment
- Kernel and loader are no_std crates
- Only std-compatible crates are unit tested
- Dependency checks ensure no accidental std usage

### Custom Build System
- Uses the `xtask` pattern for build automation
- Integration tests verify the custom build process
- QEMU integration for system testing

## Tested Crates

The following crates are tested in the CI:
- `oso_proc_macro_logic`: Procedural macro logic
- `oso_error`: Error handling
- `xtask`: Build automation tools

## Setup Requirements

To enable all CI features:

1. **GitHub Pages**: Enable in repository settings for documentation
2. **Branch Protection**: Set up branch protection rules for `main`
3. **Secrets**: No additional secrets required (uses default GITHUB_TOKEN)

## Local Testing

To run similar checks locally:

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --workspace --all-targets -- -D warnings

# Documentation
cargo doc --workspace --no-deps --document-private-items

# Build for aarch64
cd oso_kernel && cargo build --target aarch64-unknown-none-elf.json --release
cd ../oso_loader && cargo build --target aarch64-unknown-uefi --release

# Run tests
cargo test -p oso_proc_macro_logic
cargo test -p oso_error
cargo test -p xtask

# Security audit
cargo audit
```

## Customization

To modify the CI:

1. **Add new targets**: Update the `targets` field in workflow files
2. **Add new test crates**: Update the test job in `ci.yml`
3. **Change branches**: Modify the `on.push.branches` and `on.pull_request.branches`
4. **Add integration tests**: Extend the integration job with more QEMU tests
