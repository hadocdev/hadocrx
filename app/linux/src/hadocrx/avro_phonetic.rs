use std::ffi::{c_char, CStr, CString};

use ffi_convert::AsRust;

#[link(name = "hadocrx", kind = "dylib")]
unsafe extern "C" {
    #[link_name="avro_phonetic_convert"]
    fn avro_phonetic_convert(text: *const c_char) -> *const c_char;
}

pub fn convert(text: &str) -> String {
    let text_cstr = CString::new(text).unwrap_or_default();
    let output_str = unsafe {
        let cstr = avro_phonetic_convert(text_cstr.as_ptr());
        CStr::from_ptr(cstr)
    };
    output_str.as_rust().unwrap_or_default() 
}
