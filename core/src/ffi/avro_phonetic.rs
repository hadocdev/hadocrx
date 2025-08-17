use std::ffi::{CStr, CString};

use ffi_convert::AsRust;
use crate::avro_phonetic::avro_phonetic_convert_c;

pub fn convert(text: &str) -> String {
    let text_cstr = CString::new(text).unwrap_or_default();
    let output_str = unsafe {
        let cstr = avro_phonetic_convert_c(text_cstr.as_ptr());
        CStr::from_ptr(cstr)
    };
    output_str.as_rust().unwrap_or_default() 
}
