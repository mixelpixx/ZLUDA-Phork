use std::env::VarError;

fn main() -> Result<(), VarError> {
    if !cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=hipsparse");
        println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
    }
    Ok(())
}
