{ rustPlatform, pkg-config, ... }:
  rustPlatform.buildRustPackage {
    pname = "pfica-stats";
    version = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package.version;
    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;
    meta = {
      description = "Stats Database for pfings.camp";
      homepage = "https://github.com/emmaboecker/pfica-stats";
    };
    
    doCheck = false;

    nativeBuildInputs = [
      pkg-config
    ];
  }