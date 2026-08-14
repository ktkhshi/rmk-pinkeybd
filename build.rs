use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use xz2::read::XzEncoder;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));
    fs::copy("memory.x", out_dir.join("memory.x")).expect("failed to copy memory.x");

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=vial.json");
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    let vial_json =
        fs::read_to_string("vial.json").expect("vial.json is required for Vial support");
    let compact_json =
        json::stringify(json::parse(&vial_json).expect("vial.json must be valid JSON"));
    let mut compressed = Vec::new();
    XzEncoder::new(compact_json.as_bytes(), 6)
        .read_to_end(&mut compressed)
        .expect("failed to compress vial.json");

    let generated = format!(
        "pub const VIAL_KEYBOARD_DEF: &[u8] = &{compressed:?};\n        pub const VIAL_KEYBOARD_ID: &[u8] = &[0xA7, 0x4E, 0xB1, 0x20, 0xD8, 0x63, 0x9C, 0xF5];\n"
    );
    fs::write(Path::new(&out_dir).join("vial_config.rs"), generated)
        .expect("failed to write generated Vial config");
}
