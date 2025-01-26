{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, rust-overlay, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgsCross = nixpkgs.legacyPackages.${system}.pkgsCross.riscv64;
        rust-bin = rust-overlay.lib.mkRustBin { } pkgsCross.buildPackages;
        pkgs = import nixpkgs { system = "${system}"; };
      in {
        devShells = {
          default = pkgsCross.mkShell {
            nativeBuildInputs =
              [ rust-bin.stable.latest.minimal pkgsCross.pkg-config ];

            depsBuildBuild = [ pkgs.qemu ];

            env = {
              CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER =
                "${pkgsCross.stdenv.cc.targetPrefix}cc";
              CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER =
                "qemu-riscv64"; # Running in emulator
            };
          };
        };
      });
}
