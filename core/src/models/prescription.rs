#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct DrugItem {
    pub brand_name: String,
    pub generic_name: String,
    pub strength: String,
    pub formulation: String,
    pub manufacturer: String,
    pub dosing: String,
    pub instructions: String,
    pub duration: String,
}
