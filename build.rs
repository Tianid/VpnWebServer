mod build_resources;

use build_resources::generator;

fn main() {
    generator::generate("src/generated");
    println!("[BUILD INFO] Successfully generate all files");
    println!("cargo:rerun-if-changed=build_resources/*");
}
