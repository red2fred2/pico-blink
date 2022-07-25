#!/bin/sh

# Horrible workaround, but it makes unit testing a breeze
awk '{sub(/#!\[no_main\]/,"// #![no_main]")}1' src/main.rs > temp
cp temp src/main.rs
rm temp

# Build and test
cargo test_pc

# Undo workaround
awk '{sub(/\/\/ #!\[no_main\]/,"#![no_main]")}1' src/main.rs > temp
cp temp src/main.rs
rm temp
