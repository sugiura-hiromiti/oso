# CI/CD Setup for oso

This document explains the CI/CD configuration for the oso OS project, which now supports **multi-platform builds and testing**.

## Overview

The oso project now has comprehensive CI/CD with support for both **Linux (ARM64)** and **macOS** platforms, reflecting the project's use of platform-specific tools like `hdiutil` on macOS and `mount`/`losetup` on Linux.

## Workflows

### 1. Quick CI Pipeline (`.github/workflows/ci.yml`)
**Purpose**: Fast feedback for basic checks
**Triggers**: Every push/PR to `main` and `develop`
**Platform**: Linux ARM64 only
**Duration**: ~5-10 minutes

**Jobs:**
- Code formatting and basic linting
- Quick compilation checks
- Unit tests for std-compatible crates
- Security audit
- Documentation generation

### 2. Multi-Platform CI (`.github/workflows/multi-platform-ci.yml`)
**Purpose**: Comprehensive cross-platform testing
**Triggers**: Every push/PR to `main` and `develop`
**Platforms**: Linux ARM64 + macOS 14
**Duration**: ~15-25 minutes

**Jobs:**
- **Check and Lint**: Code quality checks on both platforms
- **Build**: Full aarch64 builds with artifact upload
- **Test**: Unit tests across platforms
- **Integration**: QEMU testing with platform-specific setup
- **Security**: Security audits on both platforms
- **Dependency Check**: Platform-specific dependency validation

### 3. Build and Run Tests (`.github/workflows/build-and-run.yml`)
**Purpose**: Full integration testing with QEMU
**Triggers**: Push to `main`, PR to `main`, manual dispatch
**Platforms**: Linux ARM64 + macOS 14
**Duration**: ~20-30 minutes

**Features:**
- Complete build process testing
- Platform-specific mount tool testing (`hdiutil` vs `mount`)
- QEMU boot testing with configurable timeouts
- Cross-platform build artifact comparison
- Optional extended boot testing

### 4. Multi-Platform Release (`.github/workflows/multi-platform-release.yml`)
**Purpose**: Create releases with platform-specific builds
**Triggers**: Git tags (`v*`), manual dispatch
**Platforms**: Linux ARM64 + macOS 14
**Duration**: ~25-35 minutes

**Features:**
- Platform-specific release artifacts
- Comprehensive checksums and verification
- Automated release notes generation
- Cross-platform artifact integrity checks

### 5. Documentation (`.github/workflows/docs.yml`)
**Purpose**: Deploy rustdoc to GitHub Pages
**Triggers**: Push to `main`
**Platform**: Linux ARM64
**Duration**: ~10-15 minutes

## Platform Support

### Linux (ubuntu-24.04-arm)
- **Mount tools**: `sudo mount -o loop`, `sudo umount`, `losetup`
- **QEMU**: `qemu-system-aarch64` via apt
- **Checksums**: `sha256sum`
- **Status**: Primary development platform

### macOS (macos-14)
- **Mount tools**: `hdiutil attach/detach`
- **QEMU**: `qemu-system-aarch64` via Homebrew
- **Checksums**: `shasum -a 256`
- **Status**: Full feature parity with Linux

## Key Features

### Multi-Platform Build Matrix
```yaml
strategy:
  matrix:
    os: [ubuntu-24.04-arm, macos-14]
    include:
      - os: ubuntu-24.04-arm
        platform: linux
      - os: macos-14
        platform: macos
```

### Platform-Specific Tool Testing
- **Linux**: Tests `sudo`, `mount`, `umount`, `losetup` availability
- **macOS**: Tests `hdiutil` functionality
- **Both**: QEMU installation and basic functionality

### Cross-Platform Artifact Comparison
- Compares binary sizes between platforms
- Verifies build reproducibility
- Alerts on unexpected differences

### Enhanced Caching Strategy
- Platform-specific cache keys
- Separate caches for different workflow types
- Optimized for multi-platform builds

## Project-Specific Considerations

### aarch64 Focus
- All builds target `aarch64-unknown-none-elf.json` for kernel
- UEFI target `aarch64-unknown-uefi` for loader
- QEMU aarch64 system emulation on both platforms

### no_std Environment
- Kernel and loader remain no_std
- Cross-platform dependency validation
- Platform-agnostic no_std compliance checks

### Platform-Aware Build System
- `xtask` automatically detects host platform
- Platform-specific mount/unmount operations
- Cross-platform QEMU integration

## Tested Crates

**Cross-platform testing:**
- `oso_proc_macro_logic`: Procedural macro logic
- `oso_error`: Error handling
- `xtask`: Build automation (platform detection)

**Platform-specific validation:**
- Kernel and loader build consistency
- Mount tool functionality
- QEMU integration

## Setup Requirements

### Repository Settings
1. **GitHub Pages**: Enable for documentation deployment
2. **Branch Protection**: Configure for `main` branch
3. **Actions**: Ensure both Linux ARM64 and macOS runners are available

### No Additional Secrets Required
- Uses default `GITHUB_TOKEN`
- No platform-specific credentials needed

## Local Testing

### Linux
```bash
# Platform detection
cargo run -p xtask -- --help

# Full build and test
cargo run -p xtask

# Quick checks
cargo fmt --all -- --check
cargo test -p oso_proc_macro_logic oso_error xtask
```

### macOS
```bash
# Verify hdiutil availability
hdiutil version

# Platform detection
cargo run -p xtask -- --help

# Full build and test
cargo run -p xtask

# Install QEMU if needed
brew install qemu
```

## Manual Workflow Triggers

### Extended Boot Testing
```bash
gh workflow run build-and-run.yml -f run_full_test=true
```

### Manual Release
```bash
gh workflow run multi-platform-release.yml -f tag_name=v1.0.0-beta
```

## Monitoring and Status

### Recommended Status Badges
```markdown
[![Quick CI](https://github.com/sugiura-hiromiti/oso/workflows/CI%20(Quick%20Checks)/badge.svg)](https://github.com/sugiura-hiromiti/oso/actions/workflows/ci.yml)
[![Multi-Platform](https://github.com/sugiura-hiromiti/oso/workflows/Multi-Platform%20CI/badge.svg)](https://github.com/sugiura-hiromiti/oso/actions/workflows/multi-platform-ci.yml)
```

### Key Metrics
- Cross-platform build time comparison
- Binary size consistency
- QEMU boot success rates
- Platform-specific test coverage

## Troubleshooting

### Common Platform Issues
- **macOS**: Ensure Xcode command line tools are installed
- **Linux**: Verify sudo permissions for mount operations
- **Both**: Check QEMU installation and PATH

### Debug Commands
```bash
# Check platform detection
uname -s

# Verify QEMU
qemu-system-aarch64 --version

# Test mount tools
# Linux: sudo mount --version
# macOS: hdiutil version
```

## Migration from Single-Platform

The new multi-platform setup is **backward compatible** with existing workflows. The original `ci.yml` now focuses on quick checks, while comprehensive testing moved to dedicated multi-platform workflows.

### Benefits
- **Broader compatibility**: Ensures oso works on both major development platforms
- **Better testing**: Platform-specific tool validation
- **Enhanced releases**: Platform-specific artifacts with verification
- **Developer flexibility**: Contributors can use either Linux or macOS

For detailed workflow information, see [WORKFLOWS.md](.github/WORKFLOWS.md).
