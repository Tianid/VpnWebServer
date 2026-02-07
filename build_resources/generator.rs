use std::fs;
use std::path::Path;
use crate::build_resources::resources_path_to_generate::create_resource_paths;

pub fn generate(path: &str) {
    if !Path::new(path).exists() {
        fs::create_dir(path).expect(format!("[BUILD ERROR] Failed to create`{}`", path).as_str());
    }

    generate_mod(format!("{}/mod.rs", path).as_str());
    generate_constant(format!("{}/constants.rs", path).as_str(), create_resource_paths())
}



fn generate_mod(path: &str) {
    generate_file(path, prepare_mod_content())
}

fn generate_constant(path: &str, resources: Vec<(String, String)>) {
    generate_file(path, prepare_constants_content(resources))
}

fn prepare_mod_content() -> String {
    r#"
pub mod constants;
"#.to_string()
}

fn prepare_constants_content(resources: Vec<(String, String)>) -> String {
    let mut content = String::new();
    for pair in resources {
        content += &format!("pub const {}: &str = \"{}\";\n", pair.0, pair.1);
    }

    content
}

fn generate_file(path: &str, content: String) {
    fs::write(path, content)
        .expect(format!("[BUILD ERROR] Failed to generate `{}`", path).as_str());
}
