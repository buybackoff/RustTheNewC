extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut lmdb: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    lmdb.push("lmdb");
    lmdb.push("libraries");
    lmdb.push("liblmdb");

    cc::Build::new()
        .file(lmdb.join("mdb.c"))
        .file(lmdb.join("midl.c"))
        .define("MDB_MAXKEYSIZE", Some("0")) // Set max key size to max computed value instead of default 511
        .opt_level(2) // https://github.com/LMDB/lmdb/blob/LMDB_0.9.21/libraries/liblmdb/Makefile#L25
        .static_crt(true)
        .compile("liblmdb.a");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(true)
        .use_core()
        .ctypes_prefix("libc")
        .whitelist_function("mdb_.*") // it adds recursively all used types so the next line in this case changes nothing for this particular case
        .whitelist_type("mdb_.*")
        .prepend_enum_name(false)
        .constified_enum_module("MDB_cursor_op") // allows access to enum values as MDB_cursor_op.MDB_NEXT
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to src folder to make rls autocomplete work.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Tell cargo to tell rustc to link the lmdb library.
    println!("cargo:rustc-link-lib=static=lmdb");
}
