fn main() {
    #[cfg(feature = "cpp")]
    {
        cxx_build::bridge("src/bindings/cpp.rs")
            .file("cpp/src/demo.cpp")
            .include("cpp/include")
            .std("c++17")
            .compile("cr_core_cpp");

        println!("cargo:rerun-if-changed=cpp/src/bindings/cpp.rs");
        println!("cargo:rerun-if-changed=cpp/src/demo.cpp");
        println!("cargo:rerun-if-changed=cpp/include/cr_core/demo.h");
    }
}
