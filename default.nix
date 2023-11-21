{}:
let
  rev = "fe82c7563fbcc82504be06734f16a3690243bcc5";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  # nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/master.tar.gz";
  pkgs = import nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo-leptos
    flyctl
    leptosfmt
    nodejs_21
    nodePackages_latest.postcss-cli
    rustup
  ];
}
