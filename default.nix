{}:
let
  rev = "efc8686dfc086e7cde15504bcae3e4951303486a";
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
