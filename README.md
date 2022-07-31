# Pico Blink
This serves as a template for rust projects on RP2040 based boards.
It's still a work in progress, I'm going off an example to start.

## Requirements
* Cmake
* Rustup

## Setup
Configure rust with the nightly features. (Stable may work, but nightly definitely will)
```
$ rustup default nightly
```

Add pico architecture toolchain
```
$ rustup target add thumbv6m-none-eabi
```

Initialize git submodules
```
$ git submodule update --init --recursive
```

Run config script
```
$ sh config.sh
```

## Building
As long as the setup has been done, the build script should handle everything. Just run
```
$ sh build.sh
```

## Run unit tests
Just run the test script.

The configuration assumes that this will be run on x86_64 linux, but that can be changed in `.cargo/config` under `[alias] test_pc = ...`
```
$ sh test.sh
```