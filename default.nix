{}:
let
  rev = "df7c06fe940c83d735a8d4bcfe7185d1ac9c9222";
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/${rev}.tar.gz";
  # nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/archive/master.tar.gz";
  pkgs = import nixpkgs { };
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    binaryen
    flyctl
    leptosfmt
    nodejs_21
    nodePackages_latest.tailwindcss
    cargo-leptos
    rustup
    openssl
    protobuf_21
  ] ++ (if pkgs.stdenv.isDarwin then [
      darwin.apple_sdk.frameworks.Foundation
      pkgs.darwin.libiconv
    ] else []);
}
