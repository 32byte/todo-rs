// build.rs
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/kb.c")
        .compile("libutil.a");
}
