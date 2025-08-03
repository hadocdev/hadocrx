use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CMedicineId {
    pub id: u64
}

#[repr(C)]
#[derive(Debug)]
pub struct CMedicineData {
    pub id: CMedicineId,
    pub brand_name: *mut c_char,
    pub generic_name: *mut c_char,
    pub strength: *mut c_char,
    pub formulation: *mut c_char,
    pub manufacturer: *mut c_char,
    pub dosing: *mut c_char,
    pub instructions: *mut c_char,
    pub duration: *mut c_char,
}
