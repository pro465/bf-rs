this is a compiler that first transpiles the target bf program to Rust and calls `rustc -O ~/_rust.rs` to compile transpiled program to binary excutable amd run the binary.

# warning

this would overwrite `~/_rust` and `~/_rust.rs`, so maake sure you dont have anything important in these files before running this
