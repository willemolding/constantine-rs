# Constantine-rs

A Rust wrapper crate for the constantine crypto library in Nim

## Building

This was a useful resource for designing the build process https://stackoverflow.com/questions/59879692/how-to-call-a-nim-function-from-rust-through-c-ffi

The build process transpiles the Nim code to C and then builds and links this with the Rust build. See `build.rs`.

The build script assumes that `nimbase.h` for the platform you are building for is in `/usr/lib/nim/`. If you installed Nim using choosenim then it will be in `~/.choosenim/toolchains/nim-<version>/lib/`. You can either symlink it to `/usr/lib` or change the build script.

TODO: add an env var for setting this
TODO: These are platform specific so I will need a way to swap these for cross-compiles

