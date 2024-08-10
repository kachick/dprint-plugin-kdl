{
  inputs = {
    # Candidate channels
    #   - https://github.com/kachick/anylang-template/issues/17
    #   - https://discourse.nixos.org/t/differences-between-nix-channels/13998
    # How to update the revision
    #   - `nix flake update --commit-lock-file` # https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake-update.html
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      lib = nixpkgs.lib;
      # List: https://github.com/NixOS/nixpkgs/blob/nixos-24.05/lib/systems/flake-systems.nix
      #
      # https://github.com/NixOS/nixpkgs/blob/475556854559746466df20d74eef189373816b67/flake.nix?plain=1#L11
      # https://github.com/NixOS/nixpkgs/blob/475556854559746466df20d74eef189373816b67/lib/systems/default.nix?plain=1#L48-L56
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default =
            with pkgs;
            mkShell {
              buildInputs = [
                # https://github.com/NixOS/nix/issues/730#issuecomment-162323824
                # https://github.com/kachick/dotfiles/pull/228
                bashInteractive
                findutils # xargs
                nixfmt-rfc-style
                nil
                go-task

                dprint
                typos

                rustc
                rustc.llvmPackages.lld
                rustc.llvmPackages.bintools
                rustc-wasm32
                rustc-wasm32.llvmPackages.lld
                rustc-wasm32.llvmPackages.bintools
                llvmPackages.bintools
                rustc-wasm32.llvmPackages.bintools
                cargo
                rustfmt
                rust-analyzer
                clippy
              ];

              # FIXME: error: linker `rust-lld` not found
              # https://github.com/NixOS/nixpkgs/issues/70238
              nativeBuildInputs = [
                rustc.llvmPackages.lld
                rustc.llvmPackages.bintools
                rustc-wasm32
                rustc-wasm32.llvmPackages.lld
                rustc-wasm32.llvmPackages.bintools
                llvmPackages.bintools
                rustc-wasm32.llvmPackages.bintools
                cargo-binutils # rust-lld
              ];

              # fix "linker `rust-lld` not found"
              CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
            };
        }
      );
    };
}
