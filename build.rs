use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("bindgen/bindgen.h")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("bindgen/sh2/euler.c")
        .file("bindgen/sh2/sh2_SensorValue.c")
        .file("bindgen/sh2/sh2.c")
        .file("bindgen/sh2/sh2_util.c")
        .file("bindgen/sh2/shtp.c")
        .compile("sh2");

    println!("cargo::rerun-if-changed=bindgen/bindgen.h");
    println!("cargo::rerun-if-env-changed=OUT_DIR");
}
