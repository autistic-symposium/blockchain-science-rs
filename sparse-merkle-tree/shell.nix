with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner  = "mozilla";
      repo   = "nixpkgs-mozilla";
      rev    = "e912ed483e980dfb4666ae0ed17845c4220e5e7c";
      sha256 = "08fvzb8w80bkkabc1iyhzd15f4sm7ra10jn32kfch5klgl0gj3j3";
   };
   moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
   nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
   rustChannel = nixpkgs.latest.rustChannels.stable.rust;
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;
stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    rustChannel
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];

  # Set Environment Variables
  RUST_BACKTRACE  = 1;
}

