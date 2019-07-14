extern crate blosc_sys;
extern crate lmdb_sys;
extern crate sqlite_sys;

use libc::*;

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
