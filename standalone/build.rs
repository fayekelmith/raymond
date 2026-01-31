use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Only apply embedded-specific linker args when targeting ARM
    let target = env::var("TARGET").unwrap();
    
    if target.starts_with("thumbv") || target.starts_with("arm") {
        let out = PathBuf::from(env::var("OUT_DIR").unwrap());
        fs::copy("memory.x", out.join("memory.x")).unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=memory.x");
        println!("cargo:rustc-link-arg=--nmagic");
    }
}
