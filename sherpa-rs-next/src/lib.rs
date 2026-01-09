pub mod asr;
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

#[macro_export]
macro_rules! delegate_method {
    ($method_name:ident, $param_type:ty) => {
        paste::paste! {
            pub fn [<with_ $method_name>](&mut self, param: $param_type) -> &mut Self {
                self.base.[<with_ $method_name>](param);
                self
            }
        }
    };
}
