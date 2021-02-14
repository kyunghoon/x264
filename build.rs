extern crate cc;
extern crate bindgen;

use std::process::Command;
use std::path;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").map(|s| path::PathBuf::from(s).join("x264-build")).unwrap();
    //let out_dir = std::env::current_dir().unwrap();
    let install_dir = out_dir.join("x264-build");

    let inner_dir = path::PathBuf::from("x264");

    if !Command::new("bash")
        .arg("configure")
        .arg(format!("--prefix={}", install_dir.display()))
        .arg("--enable-static")
        .current_dir(&inner_dir)
        .status().unwrap().success() {
        panic!("configure failed");
    }

    if !Command::new("make")
        .current_dir(&inner_dir)
        .status().unwrap().success() {
        panic!("make failed");
    }

    if !Command::new("make")
        .arg("install")
        .current_dir(&inner_dir)
        .status().unwrap().success() {
        panic!("make install failed");
    }

    let include_dir = install_dir.join("include");
    let lib_dir = install_dir.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=x264");
    println!("cargo:include={}", include_dir.display());
    println!("cargo:lib={}", lib_dir.display());
    //println!("cargo:rustc-link-lib=static=stdc++");

    /*
    // bindgen
    let buildver = "161";
    bindgen::Builder::default()
        .header_contents("prefix.h", "#ifndef __PREFIX_H__
        #define __PREFIX_H__
        #  include <stdint.h>
        #endif")
        .raw_line(format!(
            "pub unsafe fn x264_encoder_open(params: *mut x264_param_t) -> *mut x264_t {{
                               x264_encoder_open_{}(params)
                          }}",
            buildver
        ))
        .header("x264-build/include/x264.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("failed to generate bindings")
        .write_to_file("src/x264_bindings.rs")
        .expect("failed to write bindings");
    */
}

