extern crate blosc_sys;
extern crate lmdb_sys;
extern crate sqlite_sys;

use libc::*;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn native_versions_get() -> *mut c_char {
    let s = format!("LMDB: {}\nBlosc: {}\nSQLite: {}", lmdb_version(), blosc_version(), sqlite_version());
    let cs = CString::new(s).unwrap();
    cs.into_raw()
}

#[no_mangle]
pub extern "C" fn native_versions_free(c: *mut c_char) {
    if c.is_null() {
        return;
    }
    // CString is dropped automatically when does out of scope
    unsafe { CString::from_raw(c); }

    // Print diagnostics in debug build
    if cfg!(debug_assertions) {
        println!("Rust string is dropped");
    }
}


pub fn lmdb_version() -> String {
    unsafe {
        let mut major: c_int = Default::default();
        let mut minor: c_int = Default::default();
        let mut patch: c_int = Default::default();
        lmdb_sys::mdb_version(&mut major, &mut minor, &mut patch);
        return format!("{}.{}.{}", major, minor, patch);
    }
}

pub fn blosc_version() -> String {
    unsafe {
        let cptr = blosc_sys::blosc_get_version_string();
        return std::ffi::CStr::from_ptr(cptr).to_str().unwrap().to_owned();
    }
}

pub fn sqlite_version() -> String {
    unsafe {
        let cptr = sqlite_sys::sqlite3_libversion();
        return std::ffi::CStr::from_ptr(cptr).to_str().unwrap().to_owned();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_versions_get_works() {
        unsafe{
            let nv = super::native_versions_get();
            let cs = std::ffi::CString::from_raw(nv);
            assert_eq!(cs.to_str().unwrap(), "LMDB: 0.9.70\nBlosc: 1.15.2.dev\nSQLite: 3.29.0");
            std::mem::forget(cs);
            super::native_versions_free(nv);
        }
    }

    #[test]
    fn lmdb_works() {
        assert_eq!(super::lmdb_version(), "0.9.70");
    }

    #[test]
    fn blosc_works() {
        unsafe {
            blosc_sys::blosc_set_nthreads(6);
            let threads = blosc_sys::blosc_get_nthreads();
            assert_eq!(threads, 6);
            assert_eq!(super::blosc_version(), "1.15.2.dev");
        }
    }

    #[test]
    fn sqlite_works() {
        assert_eq!(super::sqlite_version(), "3.29.0");
    }
}
