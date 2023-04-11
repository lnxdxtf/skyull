use super::template_struct::Templates;
use std::str;

pub fn get_templates() -> Templates {
    let templates_bytes = include_bytes!("templates.json");
    let templates: Templates = serde_json::from_str(str::from_utf8(templates_bytes).unwrap()).unwrap();
    templates
}
