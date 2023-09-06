this is a compiler that first transpiles the target bf program to Rust and calls `rustc -C opt-level=3 PRNT/_rust.rs` (where PRNT is the bf file's parent directory) to compile transpiled program to binary executable amd run the binary.

# warning

this would overwrite `PRNT/_rust` and `PRNT/_rust.rs`, so make sure you dont have anything important in these files before running this
