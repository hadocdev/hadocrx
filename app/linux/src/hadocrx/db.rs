use std::ffi::{c_char, CStr, CString};

use ffi_convert::{AsRust, CStringArray};


#[link(name = "hadocrx", kind = "dylib")]
unsafe extern "C" {
    #[link_name = "get_generic_names"]
    fn get_generic_names_unsafe() -> CStringArray;
    #[link_name = "get_generic_name_by_brand_name"]
    fn get_generic_name_by_brand_name_unsafe(brand_name: *const c_char) -> *const c_char;
    #[link_name = "get_brand_names"]
    fn get_brand_names_unsafe() -> CStringArray;
}


#[allow(dead_code)]
pub fn get_generic_names() -> Vec<String> {
    let c_string_array: CStringArray = unsafe { get_generic_names_unsafe() };
    c_string_array.as_rust().unwrap() 
}

#[allow(dead_code)]
pub fn get_generic_name_by_brand_name(brand_name: String) -> String {
    let brand_name_cstr = CString::new(brand_name).unwrap_or_default();
    let generic_name_cstr = unsafe { 
        get_generic_name_by_brand_name_unsafe(
            brand_name_cstr.as_ptr()
        )
    };
    let generic_name_str = unsafe { CStr::from_ptr(generic_name_cstr) };
    generic_name_str.as_rust().unwrap()
}

#[allow(dead_code)]
pub fn get_brand_names() -> Vec<String> {
    let c_string_array: CStringArray = unsafe { get_brand_names_unsafe() };
    c_string_array.as_rust().unwrap() 
}
