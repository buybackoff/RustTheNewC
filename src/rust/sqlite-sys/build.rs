extern crate cc;

use std::env;

fn main() {
    let mut sqlite = std::path::PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    sqlite.push("sqlite");

    cc::Build::new()
        .file(sqlite.join("sqlite3.c"))
        .define("SQLITE_ENABLE_FTS5", None)
        .define("SQLITE_ENABLE_SESSION", None)
        .define("SQLITE_ENABLE_JSON1", None)
        .define("SQLITE_ENABLE_RTREE", None)
        .define("SQLITE_ENABLE_UNLOCK_NOTIFY", None)
        .define("SQLITE_THREADSAFE", Some("1"))
        .define("SQLITE_DEFAULT_MEMSTATUS", Some("0"))
        .define("SQLITE_MAX_EXPR_DEPTH", Some("0"))
        .define("SQLITE_OMIT_DEPRECATED", None)
        .define("SQLITE_DIRECT_OVERFLOW_READ", None)
        .define("SQLITE_LIKE_DOESNT_MATCH_BLOBS", None)
        .define("SQLITE_OMIT_DECLTYPE", None)
        .define("SQLITE_OMIT_PROGRESS_CALLBACK", None)
        .define("SQLITE_USE_ALLOCA", None)
        .opt_level(2)
        .static_crt(true)
        .compile("libsqlite3.a");

    // let bindings = bindgen::Builder::default()
    //     .header("wrapper.h")
    //     .generate_comments(true)
    //     .use_core()
    //     .ctypes_prefix("libc")
    //     .whitelist_function("sqlite3_.*")
    //     .whitelist_type("sqlite3_.*")
    //     .prepend_enum_name(false)
    //     .generate()
    //     .expect("Unable to generate bindings");

    // // Write the bindings to src folder to make rls autocomplete work.
    // let out_path = std::path::PathBuf::from("src");
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    // Tell cargo to tell rustc to link the lmdb library.
    println!("cargo:rustc-link-lib=static=sqlite3");
}
