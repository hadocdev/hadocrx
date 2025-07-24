use ffi_convert::{AsRust, CStringArray};


#[link(name = "hadocrx", kind = "dylib")]
unsafe extern "C" {
    #[link_name = "get_generics"]
    fn get_generics_unsafe() -> CStringArray;
}


#[allow(dead_code)]
pub fn get_generics() -> Vec<String> {
    let c_string_array: CStringArray = unsafe { get_generics_unsafe() };
    c_string_array.as_rust().unwrap() 
}
