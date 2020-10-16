use std::env;
use std::path::{PathBuf, Path};

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let platform = if cfg!(target_family = "windows") { "windows" } else { "unix" };
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        panic!("can not build on this platform.")
    };

    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cpp")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-w")
        .compile("wrapper");

    println!("cargo:rustc-link-search={}", root.join("shared/md").join(format!("{}.{}", platform, arch)).display());
    println!("cargo:rustc-link-search={}", root.join("shared/td").join(format!("{}.{}", platform, arch)).display());
    println!("cargo:rustc-link-search={}", root.join("shared/data_collect").join(format!("{}.{}", platform, arch)).display());

    if platform == "unix" {
        println!("cargo:rustc-link-lib=dylib=LinuxDataCollect");
    } else {
        println!("cargo:rustc-link-lib=dylib=WinDataCollect");
    }
    println!("cargo:rustc-link-lib=dylib=thostmduserapi_se");
    println!("cargo:rustc-link-lib=dylib=thosttraderapi_se");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");


    // ctp api header is clean enough, we will use blacklist instead whitelist
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .derive_debug(true)
        // make output smaller
        .layout_tests(false)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // we will handle class mannually by `autobind.py`
        // function defined in rust
        .opaque_type("CThostFtdcTraderApi")
        .opaque_type("CThostFtdcTraderSpi")
        .opaque_type("CThostFtdcMdApi")
        .opaque_type("CThostFtdcMdSpi")
        // remove comment to get right cpp extern from bindgen and reinoke autobind.py
        // to get right rust code.
        .blacklist_function("Rust_CThostFtdcTraderSpi_Trait.*")
        .blacklist_function("Rust_CThostFtdcMdSpi_Trait.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
