use std::{ffi::{c_char, CStr, CString}, sync::atomic::{AtomicU64, Ordering}};

#[repr(C)]
#[derive(Debug)]
pub struct CMedicineData {
    pub id: u64,
    pub brand_name: *mut c_char,
    pub generic_name: *mut c_char,
    pub strength: *mut c_char,
    pub formulation: *mut c_char,
    pub manufacturer: *mut c_char,
    pub dosing: *mut c_char,
    pub instructions: *mut c_char,
    pub duration: *mut c_char,
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub fn medicine_new(
    brand_name: *const c_char,
    generic_name: *const c_char,
    strength: *const c_char,
    formulation: *const c_char,
    manufacturer: *const c_char,
    dosing: *const c_char,
    duration: *const c_char,
    instructions: *const c_char
) -> CMedicineData {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    CMedicineData { 
        id: COUNTER.fetch_add(1, Ordering::Relaxed), 
        brand_name: unsafe { CString::from(CStr::from_ptr(brand_name)).into_raw() }, 
        generic_name: unsafe { CString::from(CStr::from_ptr(generic_name)).into_raw() }, 
        strength: unsafe { CString::from(CStr::from_ptr(strength)).into_raw() }, 
        formulation: unsafe { CString::from(CStr::from_ptr(formulation)).into_raw() }, 
        manufacturer: unsafe { CString::from(CStr::from_ptr(manufacturer)).into_raw() }, 
        dosing: unsafe { CString::from(CStr::from_ptr(dosing)).into_raw() }, 
        instructions: unsafe { CString::from(CStr::from_ptr(instructions)).into_raw() }, 
        duration: unsafe { CString::from(CStr::from_ptr(duration)).into_raw() } 
    }
}
