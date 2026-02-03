{
  inputs = {
    nixpkgs = {
      type = "git";
      url = "https://github.com/NixOS/nixpkgs";
      ref = "nixos-unstable";
      shallow = true;
    };

    systems = {
      type = "github";
      owner = "nix-systems";
      repo = "default-linux";
    };

    rust-overlay = {
      type = "github";
      owner = "oxalica";
      repo = "rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    systems,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    perSystem = attrs:
      nixpkgs.lib.genAttrs (import systems) (system:
        attrs (import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ];
        }));
  in {
    formatter = perSystem (pkgs: pkgs.alejandra);

    devShells = perSystem (pkgs: let
      inherit (pkgs.lib) attrValues;

      rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
        toolchain.default.override {
          extensions = [
            "rust-src"
          ];
        });

      nativeBuildPackages = with pkgs; [
        pkg-config
        dbus
        glib
        gtk3
        libsoup_3
        webkitgtk_4_1
        librsvg
      ];

      libraries = with pkgs; [
        webkitgtk_4_1
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        librsvg
      ];
    in {
      default = pkgs.mkShell {
        packages = attrValues {
          inherit rustToolchain;
          inherit
            (pkgs)
            gnumake
            pkg-config
            rust-analyzer
            ;

          inherit
            (pkgs)
            nodejs_latest
            ;
        };

        nativeBuildInputs = nativeBuildPackages;

        shellHook = with pkgs; ''
          export LD_LIBRARY_PATH="${
            lib.makeLibraryPath libraries
          }:$LD_LIBRARY_PATH"

          export OPENSSL_INCLUDE_DIR="${openssl.dev}/include/openssl"

          export OPENSSL_LIB_DIR="${openssl.out}/lib"

          export OPENSSL_ROOT_DIR="${openssl.out}"

          export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"

          # Needed for NVIDIA GPUs
          export WEBKIT_DISABLE_DMABUF_RENDERER=1
        '';
      };

      docs = pkgs.mkShell {
        packages = with pkgs; [
          gnumake
          graphviz
          librsvg
          pandoc
          plantuml
          texliveBasic
        ];
      };
    });
  };
}
