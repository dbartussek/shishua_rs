fn main() {
    #[cfg(feature = "__intern_c_bindings")]
    cc::Build::new()
        .file("test_c/shishua_bindings.c")
        .compile("shishua_bindings");
}
