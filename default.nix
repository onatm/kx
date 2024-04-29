{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  name = "kx";
  cargoHash = "sha256-SNV81sV9lTy/Hymy5xyve9asDIyW8ia/wHjx/JRGz2Q=";
  src = ./.;
}
