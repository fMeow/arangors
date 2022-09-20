{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , naersk
    , fenix
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      lib = nixpkgs.lib;
      pkgs = nixpkgs.legacyPackages.${system};
      rust = fenix.packages.${system}.default;
      inherit (rust) cargo rustc;
      # Get a naersk with the input rust version
      naerskWithRust = rust: naersk.lib."${system}".override {
        inherit (rust) cargo rustc;
      };
      env = with pkgs; {
        LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
        PROTOC = "${protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        OPENSSL_LIB_DIR = "${openssl.out}/lib";
        OPENSSL_ROOT_DIR = "${openssl.out}";
        OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
      };
      # Naersk using the default rust version
      buildRustProject = pkgs.makeOverridable ({ naersk ? naerskWithRust rust, ... } @ args: naersk.buildPackage ({
        buildInputs = with pkgs; [ sqlite postgresql ];
        targets = [ ];
        copyLibs = true;
        copyBins = true;
        copySources = [ "src" ];
        cargoBuildOptions = d: d ++ [ ];
        remapPathPrefix =
          true; # remove nix store references for a smaller output package
      } // env // args));

      # Load a nightly rust. The hash takes precedence over the date so remember to set it to
      # something like `lib.fakeSha256` when changing the date.
      crateName = "arangors";
      root = ./.;
      # This is a wrapper around naersk build
      # Remember to add Cargo.lock to git for naersk to work
      project = buildRustProject {
        inherit root;
      };
      # Running tests
      testProject = project.override {
        doCheck = true;
      };
    in
    {
      packages = {
        ${crateName} = project;
        "${crateName}-test" = testProject;
      };

      defaultPackage = self.packages.${system}.${crateName};

      # `nix develop`
      devShell = pkgs.mkShell (env // {
        nativeBuildInputs = [ rustc cargo ];
        buildInputs = with pkgs; [
          rust-analyzer
          clippy
          rustfmt
        ];
      });
    });
}
