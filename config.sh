#!/bin/bash

# Compile elf2uf2 firmware converter
cd submodules/pico-sdk/tools/elf2uf2
cmake .
make
