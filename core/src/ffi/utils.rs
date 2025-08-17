use std::ffi::CString;

// #[link(name = "hadocrx", kind = "dylib")]
// unsafe extern "C" {
//     #[link_name = "fuzzy_match"]
//     fn fuzzy_match_unsafe(choice: *const c_char, pattern: *const c_char) -> i64;
// }

use crate::utils::c_fuzzy_match;

#[allow(dead_code)]
pub fn fuzzy_match(choice: &str, pattern: &str) -> Option<i64> {
    let value = c_fuzzy_match(
        CString::new(choice).unwrap_or_default().as_ptr(), 
        CString::new(pattern).unwrap_or_default().as_ptr()
    );
    if value < 1 { None }
    else { Some(value) }
}
