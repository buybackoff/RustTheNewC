extern crate corelib;

fn main() -> () {
    println!("LMDB: {}", corelib::lmdb_version());
    println!("Blosc: {}", corelib::blosc_version());
    println!("SQLite: {}", corelib::sqlite_version());
}
