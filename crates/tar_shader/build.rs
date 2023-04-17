use std::fmt::Write;

use wgsl_to_wgpu::{create_shader_module, WriteOptions};

fn load_shader(name: &str) {
    let wgsl_path = format!("shaders/{name}.wgsl");
    let rust_path = format!("src/{name}.rs");

    println!("cargo:rerun-if-changed={wgsl_path}");
    let wgsl_source = std::fs::read_to_string(wgsl_path.clone()).unwrap();

    // Generate the Rust bindings and write to a file.
    let mut text = String::new();
    writeln!(&mut text, "// File automatically generated by build.rs.").unwrap();
    writeln!(&mut text, "// Changes made to this file will not be saved.").unwrap();
    text += &create_shader_module(
        &wgsl_source,
        &format!("../{wgsl_path}"),
        WriteOptions {
            derive_encase: true,
            derive_serde: true,
            derive_bytemuck: true,
            ..Default::default()
        },
    )
    .unwrap();

    std::fs::write(rust_path, text.as_bytes()).unwrap();
}

fn main() {
    load_shader("shader");
}
