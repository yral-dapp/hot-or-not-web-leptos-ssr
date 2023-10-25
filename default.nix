{}:
let
  rev = "51d906d2341c9e866e48c2efcaac0f2d70bfd43e";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  pkgs = import nixpkgs { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo-leptos
    rustup
    tailwindcss
  ];
}
