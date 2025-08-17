#![allow(dead_code)]

use std::ffi::{CStr, CString};
use ffi_convert::{AsRust, CStringArray};

use crate::db::*;

pub fn get_generic_names() -> Vec<String> {
    let c_string_array: CStringArray = get_generic_names_c();
    c_string_array.as_rust().unwrap() 
}

pub fn get_generic_name_by_brand_name(brand_name: String) -> String {
    let brand_name_cstr = CString::new(brand_name).unwrap_or_default();
    let generic_name_cstr = get_generic_name_by_brand_name_c(brand_name_cstr.as_ptr());
    let generic_name_str = unsafe { CStr::from_ptr(generic_name_cstr) };
    generic_name_str.as_rust().unwrap()
}

pub fn get_manufacturer_by_brand_name(brand_name: String) -> String {
    let brand_name_cstr = CString::new(brand_name).unwrap_or_default();
    let manufacturer_cstr = get_manufacturer_by_brand_name_c(brand_name_cstr.as_ptr());
    let manufacturer_str = unsafe { CStr::from_ptr(manufacturer_cstr) };
    manufacturer_str.as_rust().unwrap()
}

pub fn get_brand_name_by_generic_name_manufacturer_and_strength(generic_name: String, manufacturer: String, strength: String) -> Option<String> {
    let generic_name_cstr = CString::new(generic_name).unwrap_or_default();
    let manufacturer_cstr = CString::new(manufacturer).unwrap_or_default();
    let strength_cstr = CString::new(strength).unwrap_or_default();
    let brand_name_cstr = get_brand_name_by_generic_name_manufacturer_and_strength_c(
        generic_name_cstr.as_ptr(), manufacturer_cstr.as_ptr(), strength_cstr.as_ptr()
    );
    if !brand_name_cstr.is_null() {
        let brand_name_str = unsafe { CStr::from_ptr(brand_name_cstr) };
        Some(brand_name_str.as_rust().unwrap())
    } else {
        None
    }
}

pub fn get_brand_name_by_generic_name_and_manufacturer(generic_name: String, manufacturer: String) -> String {
    let generic_name_cstr = CString::new(generic_name).unwrap_or_default();
    let manufacturer_cstr = CString::new(manufacturer).unwrap_or_default();
    let brand_name_cstr = get_brand_name_by_generic_name_and_manufacturer_c(generic_name_cstr.as_ptr(), manufacturer_cstr.as_ptr());
    let brand_name_str = unsafe { CStr::from_ptr(brand_name_cstr) };
    brand_name_str.as_rust().unwrap()
}

pub fn get_formulations_by_brand_name_and_strength(brand_name: String, strength: String) -> Vec<String> {
    let brand_name_cstr = CString::new(brand_name).unwrap_or_default();
    let strength_cstr = CString::new(strength).unwrap_or_default();
    let c_string_array = get_formulations_by_brand_name_and_strength_c(
        brand_name_cstr.as_ptr(), strength_cstr.as_ptr()
    );
    c_string_array.as_rust().unwrap()
}

pub fn get_strengths_by_generic_name(generic_name: String) -> Vec<String> {
    let generic_name_cstr = CString::new(generic_name).unwrap_or_default();
    let c_string_array = get_strengths_by_generic_name_c(generic_name_cstr.as_ptr());
    c_string_array.as_rust().unwrap()
}

pub fn get_brand_names() -> Vec<String> {
    let c_string_array: CStringArray = get_brand_names_c();
    c_string_array.as_rust().unwrap() 
}

pub fn get_manufacturers() -> Vec<String> {
    let c_string_array: CStringArray = get_manufacturers_c();
    c_string_array.as_rust().unwrap() 
}

pub fn get_manufacturers_by_generic_name(generic_name: String) -> Vec<String> {
    let generic_name_cstr = CString::new(generic_name).unwrap_or_default();
    let c_string_array = get_manufacturers_by_generic_name_c(generic_name_cstr.as_ptr());
    c_string_array.as_rust().unwrap()
}
