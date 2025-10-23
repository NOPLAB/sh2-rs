use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("bindgen/bindgen.h")
        .use_core()
        .clang_arg("-Ibindgen")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .include("bindgen")
        .file("bindgen/sh2/euler.c")
        .file("bindgen/sh2/sh2_SensorValue.c")
        .file("bindgen/sh2/sh2.c")
        .file("bindgen/sh2/sh2_util.c")
        .file("bindgen/sh2/shtp.c")
        .compile("sh2");

    // Rerun if any source files change
    println!("cargo::rerun-if-changed=bindgen/bindgen.h");
    println!("cargo::rerun-if-changed=bindgen/sh2/euler.c");
    println!("cargo::rerun-if-changed=bindgen/sh2/euler.h");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2.c");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2.h");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2_err.h");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2_hal.h");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2_SensorValue.c");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2_SensorValue.h");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2_util.c");
    println!("cargo::rerun-if-changed=bindgen/sh2/sh2_util.h");
    println!("cargo::rerun-if-changed=bindgen/sh2/shtp.c");
    println!("cargo::rerun-if-changed=bindgen/sh2/shtp.h");
}
