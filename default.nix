{}:
let
  rev = "51d906d2341c9e866e48c2efcaac0f2d70bfd43e";
  # nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/master.tar.gz";
  pkgs = import nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    binaryen
    cargo-leptos
    flyctl
    leptosfmt
    nodejs_21
    nodePackages_latest.postcss-cli
    rustup
  ];
}
