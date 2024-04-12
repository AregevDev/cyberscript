# cyberscript
Rust binding to the [`cyber`](https://cyberscript.dev/) scripting language.  
This crate uses the C Embedding API to generate Rust bindings for `cyber`.

Cyber version: `0.4-dev`

### Prerequisites
- Zig
- libclang (for `bindgen`)

### Building
Currently, the `cyberscript-sys` crate downloads `cyber` from source and builds it as a static library.
Since `cyber` is still in development, the crate downloads the latest version.

It uses `bindgen` to generate the Rust bindings from `cyber.h`.  
For now, simply run `cargo test` inside `cyberscipt-sys`.

### Future Work
- A safe Rust API
- CI
- More tests
