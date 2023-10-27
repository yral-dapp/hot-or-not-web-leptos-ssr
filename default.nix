{}:
let
  rev = "75a52265bda7fd25e06e3a67dee3f0354e73243c";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  pkgs = import nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo-leptos
    flyctl
    rustup
    tailwindcss
  ];
}
