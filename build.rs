fn main() {
    #[cfg(feature = "cpp")]
    {
        cxx_build::bridge("src/bindings/cpp.rs")
            .file("cpp/demo.cc")
            .include("include")
            .std("c++17")
            .compile("cr_core_cpp");

        println!("cargo:rerun-if-changed=src/bindings/cpp.rs");
        println!("cargo:rerun-if-changed=cpp/demo.cc");
        println!("cargo:rerun-if-changed=include/cr_core/demo.h");
    }
}
