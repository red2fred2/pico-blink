#!/bin/sh

# Compile elf2uf2 firmware tool
cd submodules/pico-sdk/tools/elf2uf2
cmake .
make
