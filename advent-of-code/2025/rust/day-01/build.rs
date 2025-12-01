/// https://doc.rust-lang.org/nomicon/ffi.html#prepare-the-build-script

fn main() {
    cc::Build::new()
        .file("deps/euclid.c")
        .compile("rem_euclid");
}