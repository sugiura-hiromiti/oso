{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Core build tools
            binutils
            qemu
            
            # Additional tools that might be needed
            coreutils
            findutils
            gnused
            gnugrep
            gnutar
            gzip
            
            # Platform-specific tools
            util-linux  # for losetup on Linux (no-op on macOS)
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            # macOS-specific tools (hdiutil is built-in, no need to add)
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
            # Linux-specific tools
            mount
            umount
          ];
          
          shellHook = ''
            echo "oso development environment loaded"
            echo "Available tools:"
            echo "  - qemu-system-aarch64: $(which qemu-system-aarch64 2>/dev/null || echo 'not found')"
            echo "  - binutils: $(which readelf 2>/dev/null || echo 'not found')"
            echo "Platform: ${if pkgs.stdenv.isDarwin then "macOS" else "Linux"}"
          '';
        };
      }
    );
}
