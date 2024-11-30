fn main() {
    // Link to the Python shared library
    println!("cargo:rustc-link-lib=python3.8");

    // Use PyO3 to generate Python bindings
    pyo3_build_config::add_extension_module_link_args();

    // Use Maturin to build the Python package
    maturin::BuildOptions::default().build().unwrap();
}
