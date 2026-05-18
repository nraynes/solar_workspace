use const_format::formatcp;

const PKG: &str = env!("CARGO_PKG_NAME");

pub const BIN_RELEASE: &str = formatcp!("name: CI/CD Release

permissions:
  contents: write
  id-token: write

on:
  push:
    branches: [ \"master\" ]
  workflow_dispatch:

jobs:
  release:
    runs-on: macos-latest

    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 0
          fetch-tags: true

      - name: Build Linux Release
        run: |
          rustup target add x86_64-unknown-linux-gnu
          brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
          cargo build --release --target x86_64-unknown-linux-gnu --config 'target.x86_64-unknown-linux-gnu.linker = \"x86_64-unknown-linux-gnu-gcc\"'
          mkdir -p ./bin/linux
          cp ./target/x86_64-unknown-linux-gnu/release/{PKG} ./bin/linux/{PKG}

      - name: Build Arm MacOS Release
        run: |
          rustup target add aarch64-apple-darwin
          cargo build --release --target aarch64-apple-darwin
          mkdir -p ./bin/arm-macos
          cp ./target/aarch64-apple-darwin/release/{PKG} ./bin/arm-macos/{PKG}

      - name: Build Intel MacOS Release
        run: |
          rustup target add x86_64-apple-darwin
          cargo build --release --target x86_64-apple-darwin
          mkdir -p ./bin/intel-macos
          cp ./target/x86_64-apple-darwin/release/{PKG} ./bin/intel-macos/{PKG}

      - name: Build Windows Release
        run: |
          rustup target add x86_64-pc-windows-gnu
          brew install min gw-w64
          cargo build --release --target x86_64-pc-windows-gnu --config 'target.x86_64-pc-windows-gnu.linker = \"x86_64-w64-mingw32-gcc\"'
          mkdir -p ./bin/windows
          cp ./target/x86_64-pc-windows-gnu/release/{PKG}.exe ./bin/windows/{PKG}.exe

      - name: Update Release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: ./.release/semver-release
");
