pub mod vad;

#[macro_export]
macro_rules! as_c_string {
    ($a:ident) => {
        std::ffi::CString::new($a).unwrap()
    };
    ($a:expr) => {
        std::ffi::CString::new($a).unwrap()
    };
}

#[macro_export]
macro_rules! const_ptr_to_string {
    ($a:ident) => {
        std::ffi::CStr::from_ptr($a).to_string_lossy().into_owned()
    };
    ($a:expr) => {
        unsafe { std::ffi::CStr::from_ptr($a).to_string_lossy().into_owned() }
    };
    ($a:ident, $def:literal) => {
        Ok(std::ffi::CStr::from_ptr(schema)
            .to_str()
            .map_or($def, |x| x))
    };
}
