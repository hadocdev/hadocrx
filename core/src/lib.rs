use std::{ffi::{c_char, CStr}, sync::OnceLock};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

static MATCHER: OnceLock<SkimMatcherV2> = OnceLock::new();

#[allow(dead_code)]
fn get_matcher() -> &'static SkimMatcherV2 {
    MATCHER.get_or_init(|| SkimMatcherV2::default())
}

#[allow(dead_code)]
#[unsafe(no_mangle)]
pub extern "C" fn fuzzy_match(choice: *const c_char, pattern: *const c_char) -> i64 {
    let matcher = get_matcher();
    unsafe {
        let score = matcher.fuzzy_match(CStr::from_ptr(choice).to_str().unwrap(), CStr::from_ptr(pattern).to_str().unwrap());
        score.unwrap_or_else(|| -1)
    }
}
