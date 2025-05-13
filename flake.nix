{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      lib = nixpkgs.lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-tree);
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {

            env = {
              # Fix nixd pkgs versions in the inlay hints
              NIX_PATH = "nixpkgs=${pkgs.path}";
              # For vscode typos extension
              TYPOS_LSP_PATH = lib.getExe pkgs.typos-lsp;
            };

            buildInputs = with pkgs; [
              bashInteractive
              findutils # xargs
              nixfmt-rfc-style
              nixd
              go-task
              typos

              dprint
              rustc
              cargo
              rustfmt
              rust-analyzer
              clippy
            ];

            nativeBuildInputs = with pkgs; [
              rustc-wasm32.llvmPackages.bintools # rust-lld
            ];

            # Needed for avoiding "error: linker `rust-lld` not found".
            # Adding packages like binutils is not enough
            #
            # https://github.com/NixOS/nixpkgs/issues/70238
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
          };
        }
      );
    };
}
