{ pkgs, lib, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs;
    [
      cargo-watch
      clang
      gcc
      lld
      llvmPackages.libcxxStdenv
      llvmPackages.libclang
      vulkan-headers
      vulkan-loader
      clang
      pkg-config
      boost
    ] ++ lib.optionals pkgs.hostPlatform.isDarwin [
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.CoreFoundation
      darwin.apple_sdk.frameworks.SystemConfiguration
      darwin.apple_sdk.frameworks.AGL
      darwin.apple_sdk.frameworks.CoreGraphics
      darwin.apple_sdk.frameworks.GameKit
      darwin.apple_sdk.frameworks.Carbon
    ];

  # https://devenv.sh/scripts/
  scripts.hello.exec = "echo hello from $GREET";

  languages.rust = {
    enable = true;
    channel = "nightly";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  enterShell = ''
    hello
    git --version
  '';

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
  # https://devenv.sh/languages/
  # languages.nix.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
