with import <nixpkgs> {};
with lib;

let

  cleanAppFilter = path: type:
    let
      relPath = removePrefix (toString ./. + "/") (toString path);
      baseName = baseNameOf relPath;
    in
      (
      hasPrefix "src" relPath ||
      baseName == "Cargo.toml" ||
      baseName == "Cargo.lock"
      );

  cleanApp = cleanSourceWith {
    filter = cleanAppFilter;
    src = cleanSource ./.;
  };
in

rustPlatform.buildRustPackage rec {
  name = "ped2-${version}";
  version = "v1.0";

  src = cleanApp;
  cargoSha256 = "0jacm96l1gw9nxwavqi1x4669cg6lzy9hr18zjpwlcyb3qkw9z7f";
}
