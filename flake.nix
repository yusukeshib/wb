{
  description = "wb — git-branch interface backed by git-worktree";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          wb = pkgs.rustPlatform.buildRustPackage {
            pname = "wb";
            version = "0.2.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            meta = with pkgs.lib; {
              description = "git-branch interface backed by git-worktree";
              homepage = "https://github.com/yusukeshib/wb";
              license = licenses.mit;
              mainProgram = "wb";
            };
          };
          default = self.packages.${system}.wb;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      });
}
