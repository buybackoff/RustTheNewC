extern crate corelib;

fn main() -> () {
    unsafe{
        let nv = corelib::native_versions_get();
        let cs = std::ffi::CString::from_raw(nv);
        println!("{}", cs.to_str().unwrap());
        std::mem::forget(cs);  // Do not drop, we need return it back to destructor to simulate desired behavior
        corelib::native_versions_free(nv);
    }
}
