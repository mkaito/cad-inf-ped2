with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "env";
  buildInputs = [
    zsh
    rustc
    cargo
    rustracer
  ];

  RUST_BACKTRACE = 1;
}
