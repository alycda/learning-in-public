fn main() {
    // Compile the C code
    cc::Build::new()
        .file("src/aoc_ffi.c")
        .compile("aoc_ffi");

    // Tell cargo to rerun if the C files change
    println!("cargo:rerun-if-changed=src/aoc_ffi.c");
    println!("cargo:rerun-if-changed=src/aoc_ffi.h");
}
