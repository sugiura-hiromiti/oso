# oso CI/CD Workflows

This document describes the CI/CD workflows for the oso project, which now supports multi-platform builds and testing.

## Workflow Overview

### 1. Quick CI (`ci.yml`)
**Triggers**: Every push/PR to `main` and `develop`
**Purpose**: Fast feedback for basic checks
**Runs on**: Linux ARM64 only
**Duration**: ~5-10 minutes

- Code formatting checks
- Basic compilation
- Unit tests for std-compatible crates
- Security audit
- Documentation generation

### 2. Multi-Platform CI (`multi-platform-ci.yml`)
**Triggers**: Every push/PR to `main` and `develop`
**Purpose**: Comprehensive testing across platforms
**Runs on**: Linux ARM64 + macOS 14
**Duration**: ~15-25 minutes

- All checks from Quick CI, but on both platforms
- Cross-platform build verification
- Platform-specific dependency checks
- Artifact comparison between platforms

### 3. Build and Run Tests (`build-and-run.yml`)
**Triggers**: Push to `main`, PR to `main`, manual dispatch
**Purpose**: Full integration testing with QEMU
**Runs on**: Linux ARM64 + macOS 14
**Duration**: ~20-30 minutes

- Complete build process testing
- QEMU boot testing (with timeouts)
- Platform-specific mount tool testing
- Build artifact verification and comparison
- Optional full boot testing (manual trigger)

### 4. Multi-Platform Release (`multi-platform-release.yml`)
**Triggers**: Git tags (`v*`), manual dispatch
**Purpose**: Create releases with platform-specific builds
**Runs on**: Linux ARM64 + macOS 14
**Duration**: ~25-35 minutes

- Builds release artifacts for both platforms
- Creates platform-specific tarballs with checksums
- Generates comprehensive release notes
- Verifies artifact integrity

### 5. Documentation (`docs.yml`)
**Triggers**: Push to `main`
**Purpose**: Deploy rustdoc to GitHub Pages
**Runs on**: Linux ARM64
**Duration**: ~10-15 minutes

- Generates comprehensive documentation
- Deploys to GitHub Pages

## Platform Support

### Linux (ubuntu-24.04-arm)
- **Mount tool**: `sudo mount -o loop` / `sudo umount`
- **QEMU**: `qemu-system-aarch64` via apt
- **Checksums**: `sha256sum`
- **Primary development platform**

### macOS (macos-14)
- **Mount tool**: `hdiutil attach/detach`
- **QEMU**: `qemu-system-aarch64` via Homebrew
- **Checksums**: `shasum -a 256`
- **Full feature parity with Linux**

## Workflow Features

### Caching Strategy
- Cargo registry and git cache
- Platform-specific cache keys
- Target directory caching
- Separate caches for different workflow types

### Artifact Management
- Build artifacts uploaded with platform identification
- Cross-platform build comparison
- Retention policies (3-30 days depending on workflow)
- Checksum verification for releases

### Testing Strategy
- **Unit tests**: std-compatible crates only
- **Integration tests**: QEMU boot testing with timeouts
- **Platform tests**: Mount tool and dependency verification
- **Security tests**: `cargo audit` on both platforms

### Error Handling
- Timeouts for QEMU tests to prevent hanging
- Graceful failure handling for platform-specific tools
- Artifact verification before release creation

## Manual Triggers

### Full Boot Test
```bash
# Trigger via GitHub UI or:
gh workflow run build-and-run.yml -f run_full_test=true
```

### Manual Release
```bash
# Create a manual release:
gh workflow run multi-platform-release.yml -f tag_name=v1.0.0-beta
```

## Monitoring

### Status Badges
Add these to your README.md:

```markdown
[![CI](https://github.com/sugiura-hiromiti/oso/workflows/CI%20(Quick%20Checks)/badge.svg)](https://github.com/sugiura-hiromiti/oso/actions/workflows/ci.yml)
[![Multi-Platform CI](https://github.com/sugiura-hiromiti/oso/workflows/Multi-Platform%20CI/badge.svg)](https://github.com/sugiura-hiromiti/oso/actions/workflows/multi-platform-ci.yml)
[![Build and Run](https://github.com/sugiura-hiromiti/oso/workflows/Build%20and%20Run%20Tests/badge.svg)](https://github.com/sugiura-hiromiti/oso/actions/workflows/build-and-run.yml)
```

### Key Metrics to Monitor
- Build time differences between platforms
- Binary size consistency across platforms
- Test success rates
- QEMU boot success rates

## Troubleshooting

### Common Issues

1. **QEMU timeout**: Increase timeout values in workflow files
2. **Platform-specific failures**: Check tool availability in workflow logs
3. **Cache issues**: Clear cache by updating cache key in workflow
4. **Artifact size**: Monitor binary sizes in build comparison job

### Debug Commands

```bash
# Test locally on macOS
cargo run -p xtask -- --dry-run

# Test locally on Linux
sudo cargo run -p xtask -- --dry-run

# Check platform detection
cargo run -p xtask -- --help
```

## Future Enhancements

- [ ] Windows support (requires significant xtask changes)
- [ ] Performance benchmarking across platforms
- [ ] Automated dependency updates
- [ ] Hardware-in-the-loop testing
- [ ] Binary size regression tracking
- [ ] Boot time performance monitoring
