{}:
let
  rev = "51a01a7e5515b469886c120e38db325c96694c2f";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  # nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/master.tar.gz";
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
