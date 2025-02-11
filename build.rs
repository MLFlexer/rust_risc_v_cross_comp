use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it.
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rerun-if-changed=build.rs");

    // Generate bindings for C code
    let bindings = bindgen::Builder::default()
        .header("src/hello_from_c.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    // hello_from_c compile and link
    let c_file = "hello_from_c";
    let target_c_dir = PathBuf::from("target/c");
    if !target_c_dir.exists() {
        fs::create_dir_all(&target_c_dir).unwrap();
    }
    println!("cargo:rerun-if-changed=src/{}.c", c_file);
    Command::new("riscv64-none-elf-gcc")
        .arg("-c")
        .arg(format!("src/{c_file}.c"))
        .arg("-o")
        .arg(format!("target/c/{c_file}.o"))
        .status()
        .expect(&format!("Failed to compile {c_file}.c"));

    Command::new("riscv64-none-elf-ar")
        .arg("rcs")
        .arg(format!("target/c/lib{c_file}.a"))
        .arg(format!("target/c/{c_file}.o"))
        .status()
        .expect(&format!("Failed to create static library from {c_file}.o"));

    println!("cargo:rustc-link-lib=static={}", c_file);
    println!("cargo:rustc-link-search=native=target/c");
}
