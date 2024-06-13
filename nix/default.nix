{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "dotsync-env";

  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.openssl
    pkgs.git
    pkgs.curl
  ];

  shellHook = ''
    export RUST_LOG=info
    export PATH=$HOME/.cargo/bin:$PATH
    echo "Environment setup for DotSync"
  '';
}
