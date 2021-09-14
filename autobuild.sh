#!/bin/bash
echo -e "Building for raspberry pi..."
cross build --release --target armv7-unknown-linux-gnueabihf
echo -e "Building for arm64 linux..."
cross build --release --target aarch64-unknown-linux-gnu
echo -e "Building for M1 macOS"
cargo build --release --target aarch64-apple-darwin
echo -e "Building for Intel macOS"
cargo build --release --target x86_64-apple-darwin
echo -e "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu
echo -e "Building for native..."
cargo build --release
cp ./target/armv7-unknown-linux-gnueabihf/release/primesrs ./binaries/raspi-primes.elf
cp ./target/aarch64-apple-darwin/release/primesrs ./binaries/aarch64-primes.app
cp ./target/x86_64-apple-darwin/release/primesrs ./binaries/x64-primes.app
cp ./target/x86_64-pc-windows-gnu/release/primesrs.exe ./binaries/x64-primes.exe
cp ./target/aarch64-unknown-linux-gnu/release/primesrs ./binaries/aarch64-primes.elf
cp ./target/release/primesrs ./binaries/x64-primes.elf
