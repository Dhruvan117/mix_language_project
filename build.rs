fn main() {
    cc::Build::new()
        .file("sum.c")
        .compile("sum");
}
