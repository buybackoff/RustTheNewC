extern crate cmake;
extern crate bindgen;

fn main() {
    // TODO deduplicate code, only MinGW line for Windows, check if on MSVC
    if cfg!(windows) {
        if cfg!(target_env = "gnu") {
            let dst = cmake::Config::new("c-blosc")
                .generator("MinGW Makefiles")
                .define("BUILD_STATIC", "ON")
                .define("BUILD_SHARED", "OFF")
                .define("BUILD_TESTS", "OFF")
                .define("BUILD_BENCHMARKS", "OFF")
                .define("DEACTIVATE_AVX2", "OFF")
                .define("DEACTIVATE_LZ4", "OFF")
                .define("DEACTIVATE_SNAPPY", "ON")
                .define("DEACTIVATE_ZLIB", "OFF")
                .define("DEACTIVATE_ZSTD", "OFF")
                .define("PREFER_EXTERNAL_LZ4", "OFF")
                .define("PREFER_EXTERNAL_SNAPPY", "OFF")
                .define("PREFER_EXTERNAL_ZLIB", "OFF")
                .define("PREFER_EXTERNAL_ZSTD", "OFF")
                .build();
            println!("cargo:rustc-link-search=native={}/lib", dst.display());
            println!("cargo:rustc-link-lib=static=blosc");
        }else{
            let dst = cmake::Config::new("c-blosc")
                .define("BUILD_STATIC", "ON")
                .define("BUILD_SHARED", "OFF")
                .define("BUILD_TESTS", "OFF")
                .define("BUILD_BENCHMARKS", "OFF")
                .define("DEACTIVATE_AVX2", "OFF")
                .define("DEACTIVATE_LZ4", "OFF")
                .define("DEACTIVATE_SNAPPY", "ON")
                .define("DEACTIVATE_ZLIB", "OFF")
                .define("DEACTIVATE_ZSTD", "OFF")
                .define("PREFER_EXTERNAL_LZ4", "OFF")
                .define("PREFER_EXTERNAL_SNAPPY", "OFF")
                .define("PREFER_EXTERNAL_ZLIB", "OFF")
                .define("PREFER_EXTERNAL_ZSTD", "OFF")
                // .define("CMAKE_BUILD_TYPE", "Release") - cmake-rs does this right depending on opt-level and debug/release
                .static_crt(true)
                .build();
            println!("cargo:rustc-link-search=native={}/lib", dst.display());
            println!("cargo:rustc-link-lib=static=libblosc");
        }
    } else {
        let dst = cmake::Config::new("c-blosc")
            .define("BUILD_STATIC", "ON")
            .define("BUILD_SHARED", "OFF")
            .define("BUILD_TESTS", "OFF")
            .define("BUILD_BENCHMARKS", "OFF")
            .define("DEACTIVATE_AVX2", "OFF")
            .define("DEACTIVATE_LZ4", "OFF")
            .define("DEACTIVATE_SNAPPY", "ON")
            .define("DEACTIVATE_ZLIB", "OFF")
            .define("DEACTIVATE_ZSTD", "OFF")
            .define("PREFER_EXTERNAL_LZ4", "OFF")
            .define("PREFER_EXTERNAL_SNAPPY", "OFF")
            .define("PREFER_EXTERNAL_ZLIB", "OFF")
            .define("PREFER_EXTERNAL_ZSTD", "OFF")
            .build();
        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-lib=static=blosc");
    }

    // let bindings = bindgen::Builder::default()
    //     .header("wrapper.h")
    //     .use_core()
    //     .ctypes_prefix("libc")
    //     .whitelist_function(".*compress.*")
    //     .whitelist_function(".*shuffle.*")
    //     .whitelist_function(".*threads.*")
    //     .whitelist_function(".*version.*")
    //     .generate()
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("src/bindings.rs"))
    //     .expect("Couldn't write bindings!");
}