#!/bin/sh

# Build Rust
cargo build --release

# Use elf2uf2 to convert elf binary to uf2 that the RP2040 can understand
./submodules/pico-sdk/tools/elf2uf2/elf2uf2 target/thumbv6m-none-eabi/release/blink target/thumbv6m-none-eabi/release/firmware.uf2
