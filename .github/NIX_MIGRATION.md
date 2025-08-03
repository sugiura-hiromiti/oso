# Nix Migration for GitHub Actions

This document explains the migration from Homebrew to Nix for dependency management in GitHub Actions workflows.

## Changes Made

### 1. Updated `flake.nix`

Enhanced the Nix flake to include all necessary dependencies:

- **Core tools**: `binutils`, `qemu`
- **Utility tools**: `coreutils`, `findutils`, `gnused`, `gnugrep`, `gnutar`, `gzip`
- **Platform-specific tools**:
  - Linux: `util-linux` (for `losetup`), `mount`, `umount`
  - macOS: Uses built-in tools like `hdiutil`

Added a helpful shell hook that displays available tools and platform information.

### 2. Workflow Updates

All workflows now use Nix instead of Homebrew/apt-get for dependency management:

#### `ci.yml`
- Replaced `sudo apt-get install qemu-system-aarch64` with Nix setup
- All build commands now run within `nix develop --command`

#### `build-and-run.yml`
- Removed Homebrew installation steps for both Linux and macOS
- Removed hardcoded PATH manipulation for `/opt/homebrew/opt/binutils/bin`
- All build and test commands now use Nix environment

#### `multi-platform-ci.yml`
- Replaced platform-specific QEMU installation with unified Nix approach
- All build and test commands now run within Nix environment

#### `multi-platform-release.yml`
- Removed Homebrew installation steps
- All build commands now use Nix environment
- QEMU version detection updated to use Nix

#### `release.yml`
- Replaced `sudo apt-get install qemu-system-aarch64` with Nix setup
- All build commands now use Nix environment

#### `docs.yml`
- No changes needed (doesn't require QEMU or binutils)

### 3. Benefits of the Migration

1. **Consistency**: Same dependencies across all platforms (Linux/macOS)
2. **Reproducibility**: Exact same versions of tools on all platforms
3. **Simplicity**: No platform-specific installation logic needed
4. **Integration**: Respects existing `direnv` and `devshell` setup
5. **Maintenance**: Single source of truth for dependencies in `flake.nix`

### 4. How It Works

Each workflow now:

1. Installs Nix using `cachix/install-nix-action@v24`
2. Runs commands within the Nix development shell using `nix develop --command`
3. Automatically gets all dependencies defined in `flake.nix`

### 5. Local Development

Developers can continue using the existing setup:

```bash
# With direnv (automatic)
cd oso  # Environment loads automatically

# Manual activation
nix develop

# Run commands
cargo xt
```

### 6. Verification

To verify the migration works:

1. Check that all workflows pass
2. Verify that both Linux and macOS builds produce identical artifacts
3. Confirm that QEMU and binutils are available in all environments

## Troubleshooting

If issues arise:

1. **Nix installation fails**: Check GitHub token permissions
2. **Tools not found**: Verify `flake.nix` includes the required packages
3. **Platform differences**: Check if platform-specific packages are needed

## Future Improvements

Consider adding:
- Nix binary cache for faster CI builds
- Additional development tools to the flake
- Version pinning for more reproducible builds
