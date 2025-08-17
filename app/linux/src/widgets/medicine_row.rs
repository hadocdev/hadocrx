use hadocrx::ffi::prescription::MedicineData;
use super::custom_icon_button;

#[derive(Debug)]
pub struct MedicineRow {
    pub data: MedicineData,
    pub label_formulation: gtk::Label,
    pub label_brand_name: gtk::Label,
    pub label_generic_name: gtk::Label,
    pub label_strength: gtk::Label,
    pub label_dosing: gtk::Label,
    pub label_duration: gtk::Label,
    pub label_instructions: gtk::Label,
    
    pub btn_up: gtk::Button,
    pub btn_down: gtk::Button,
    pub btn_delete: gtk::Button,
}

impl MedicineRow {
    pub fn new(medicine_data: MedicineData) -> Self {
        let data = medicine_data;
        let label_formulation = gtk::Label::builder().label(&data.formulation).halign(gtk::Align::Start).build();
        let label_brand_name = gtk::Label::builder().label(&data.brand_name).css_classes(["heading"]).halign(gtk::Align::Start).build();
        let label_generic_name = gtk::Label::builder().label(&data.generic_name).css_classes(["caption"]).halign(gtk::Align::Start).build();
        let label_strength = gtk::Label::builder().label(&data.strength).halign(gtk::Align::Start).build();
        let label_dosing = gtk::Label::builder().label(&data.dosing).selectable(true).halign(gtk::Align::Start).build();
        let label_duration = gtk::Label::builder().label(&data.duration).selectable(true).halign(gtk::Align::Start).build();
        let label_instructions = gtk::Label::builder().label(&data.instructions).selectable(true).halign(gtk::Align::Start).build();
       
        let btn_up = custom_icon_button("fa-arrow-up"); 
        let btn_down = custom_icon_button("fa-arrow-down");
        let btn_delete = custom_icon_button("fa-arrow-delete");

        Self { 
            data,
            label_formulation, 
            label_brand_name, 
            label_generic_name, 
            label_strength,
            label_dosing,
            label_duration,
            label_instructions,
            btn_up, 
            btn_down,
            btn_delete 
        }
    }
}
