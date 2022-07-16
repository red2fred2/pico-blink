#!/bin/sh

# Run clean script before nuking it from orbit
./clean.sh

# Clean elf2uf2
cd submodules/pico-sdk/tools/elf2uf2

rm -rf \
boot_uf2_headers \
CMakeFiles \
cmake_install.cmake \
CMakeCache.txt \
elf2uf2 \
Makefile

# Nuke git
git clean -f
git submodule foreach git clean -f
