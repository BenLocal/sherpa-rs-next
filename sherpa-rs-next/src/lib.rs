pub mod asr;
pub mod audio;
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
        if $a.is_null() {
            $def
        } else {
            unsafe { std::ffi::CStr::from_ptr($a).to_string_lossy().into_owned() }
        }
    };
    ($a:expr, $def:expr) => {{
        let ptr: *const i8 = $a as *const i8;
        if ptr.is_null() {
            $def
        } else {
            unsafe {
                std::ffi::CStr::from_ptr(ptr)
                    .to_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|_| $def)
            }
        }
    }};
}
