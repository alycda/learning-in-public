fn main() {
    // Compile the C code
    cc::Build::new()
        .file("src/aoc_ffi.c")
        .compile("aoc_ffi");

    // Compile uthash wrapper
    cc::Build::new()
        .file("src/uthash_wrapper.c")
        .include("vendor") // Add vendor dir for uthash.h
        .compile("uthash_wrapper");

    // Generate UniFFI bindings
    uniffi::generate_scaffolding("src/aoc_ffi_day01.udl")
        .expect("Failed to generate UniFFI scaffolding");

    // Tell cargo to rerun if the C files change
    println!("cargo:rerun-if-changed=src/aoc_ffi.c");
    println!("cargo:rerun-if-changed=src/aoc_ffi.h");
    println!("cargo:rerun-if-changed=src/uthash_wrapper.c");
    println!("cargo:rerun-if-changed=src/uthash_wrapper.h");
    println!("cargo:rerun-if-changed=vendor/uthash.h");
    println!("cargo:rerun-if-changed=src/aoc_ffi_day01.udl");
}
