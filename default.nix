{}:
let
  rev = "171812ea70daf7636b2f3e25d5e6d3f5451e0496";
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
    nodePackages_latest.tailwindcss
    rustup
    # only used for clippy
    openssl
  ];
}
