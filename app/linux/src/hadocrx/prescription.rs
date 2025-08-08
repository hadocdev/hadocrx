use std::ffi::{c_char, CStr, CString};

use ffi_convert::AsRust;

#[repr(C)]
#[allow(dead_code)]
struct CMedicineData {
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
#[derive(Debug)]
pub struct MedicineData {
    pub id: u64,
    pub brand_name: String,
    pub generic_name: String,
    pub strength: String,
    pub formulation: String,
    pub manufacturer: String,
    pub dosing: String,
    pub instructions: String,
    pub duration: String,
}

impl AsRust<MedicineData> for CMedicineData {
    fn as_rust(&self) -> Result<MedicineData, ffi_convert::AsRustError> {
        let medicine_data = MedicineData {
            id: self.id,
            brand_name: unsafe { CStr::from_ptr(self.brand_name).as_rust().unwrap_or_default() }, 
            generic_name: unsafe { CStr::from_ptr(self.generic_name).as_rust().unwrap_or_default() }, 
            strength: unsafe { CStr::from_ptr(self.strength).as_rust().unwrap_or_default() }, 
            formulation: unsafe { CStr::from_ptr(self.formulation).as_rust().unwrap_or_default() }, 
            manufacturer: unsafe { CStr::from_ptr(self.manufacturer).as_rust().unwrap_or_default() }, 
            dosing: unsafe { CStr::from_ptr(self.dosing).as_rust().unwrap_or_default() }, 
            instructions: unsafe { CStr::from_ptr(self.instructions).as_rust().unwrap_or_default() }, 
            duration: unsafe { CStr::from_ptr(self.duration).as_rust().unwrap_or_default() }
        };
        Ok(medicine_data)
    }
}

impl Drop for CMedicineData {
    fn drop(&mut self) {
        let _ = self.id;
        let _ = self.brand_name;
        let _ = self.generic_name;
        let _ = self.strength;
        let _ = self.formulation;
        let _ = self.dosing;
        let _ = self.instructions;
        let _ = self.duration;
    }
}

#[link(name = "hadocrx", kind = "dylib")]
unsafe extern "C" {
    #[link_name = "medicine_new"]
    fn medicine_new(brand_name: *const c_char,
        generic_name: *const c_char,
        strength: *const c_char,
        formulation: *const c_char,
        manufacturer: *const c_char,
        dosing: *const c_char,
        duration: *const c_char,
        instructions: *const c_char
    ) -> CMedicineData;
}

#[allow(dead_code)]
impl MedicineData {
    pub fn new(
        brand_name: String,
        generic_name: String,
        strength: String,
        formulation: String,
        manufacturer: String,
        dosing: String,
        instructions: String,
        duration: String,
    ) -> Self {
        let c_medicine_data = unsafe { 
            medicine_new(
                CString::new(brand_name).unwrap_or_default().as_ptr(), 
                CString::new(generic_name).unwrap_or_default().as_ptr(), 
                CString::new(strength).unwrap_or_default().as_ptr(), 
                CString::new(formulation).unwrap_or_default().as_ptr(), 
                CString::new(manufacturer).unwrap_or_default().as_ptr(), 
                CString::new(dosing).unwrap_or_default().as_ptr(), 
                CString::new(duration).unwrap_or_default().as_ptr(), 
                CString::new(instructions).unwrap_or_default().as_ptr()
            ) 
        };
        c_medicine_data.as_rust().unwrap()
    }
}
