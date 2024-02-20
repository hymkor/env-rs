use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let mut file = File::create(&dest_path).unwrap();

    write!(
        file,
        "pub const VERSION: &str = \"v{}-{}-{}\";\n",
        version, os, arch
    )
    .unwrap();
}
