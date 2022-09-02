fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../codegen");
    println!("cargo:rerun-if-changed=../lib");
    println!("cargo:rerun-if-changed=../service");
    println!("cargo:rerun-if-changed=../model");

    codegen::main().unwrap()
}
