this is a compiler that first transpiles the target bf program to Rust and calls `rustc -C opt-level=3 ~/_rust.rs` to compile transpiled program to binary excutable amd run the binary.

# warning

this would overwrite `~/_rust` and `~/_rust.rs`, so make sure you dont have anything important in these files before running this
