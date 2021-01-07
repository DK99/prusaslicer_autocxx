fn main() {
    // It's necessary to use an absolute path here because the
    // C++ codegen and the macro codegen appears to be run from different
    // working directories.
    let root = std::path::PathBuf::from(".").canonicalize().unwrap();
    let root_src = std::path::PathBuf::from("PrusaSlicer/src").canonicalize().unwrap();
    let build_libslic3r = std::path::PathBuf::from("PrusaSlicer/build/src/libslic3r").canonicalize().unwrap();
    let eigen_src = std::path::PathBuf::from("PrusaSlicer/src/eigen").canonicalize().unwrap();

    let mut b = autocxx_build::build("src/main.rs", &[&root, &root_src, &build_libslic3r, &eigen_src]).unwrap();

    b.flag_if_supported("-std=c++17");
    b.warnings(false);
    b.compile("cxx_tests");

    println!("cargo:rerun-if-changed=src/main.rs");
}