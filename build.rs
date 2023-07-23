//Original version: https://github.com/servo/webxr

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mlsdk = env::var("MLSDK").unwrap();

    let mut builder = bindgen::Builder::default()
        .clang_arg("-v")
        .clang_args(&[
            format!("--no-standard-includes"),
            format!("--sysroot={}", mlsdk),
            format!("-I{}/include", mlsdk),
            format!("-I{}/lumin/usr/include", mlsdk),
            format!("-I{}/tools/toolchains/lib64/clang/3.8/include", mlsdk),
            format!("-I{}/lumin/usr/include/vulkan", mlsdk),
        ])
        .header("src/magicleap_c_api.h")
        .blocklist_type("MLResult")
        .size_t_is_usize(true)
        .derive_default(true)
        .rustfmt_bindings(true);

    if let Ok(flags) = env::var("CFLAGS") {
        for flag in flags.split_whitespace() {
            builder = builder.clang_arg(flag);
        }
    }

    if let Ok(flags) = env::var("CLANGFLAGS") {
        for flag in flags.split_whitespace() {
            builder = builder.clang_arg(flag);
        }
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(&out_dir);
    bindings
        .write_to_file(out_path.join("magicleap_c_api.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .archiver(Path::new(
            format!(
                "{}/tools/toolchains/bin/aarch64-linux-android-gcc-ar",
                mlsdk
            )
            .as_str(),
        ))
        .compiler(Path::new(
            format!(
                "{}/tools/toolchains/bin/aarch64-linux-android-gcc-4.9",
                mlsdk
            )
            .as_str(),
        ))
        .target("aarch64-linux-android")
        .file("src\\magicleap_c_api_wrapper.c")
        .include(Path::new(format!("{}/include", mlsdk).as_str()))
        .include(Path::new(format!("{}/lumin/usr/include", mlsdk).as_str()))
        .compile("magicleap_c_api_wrapper");

    cc::Build::new()
        .archiver(Path::new(
            format!(
                "{}/tools/toolchains/bin/aarch64-linux-android-gcc-ar",
                mlsdk
            )
            .as_str(),
        ))
        .compiler(Path::new(
            format!(
                "{}/tools/toolchains/bin/aarch64-linux-android-gcc-4.9",
                mlsdk
            )
            .as_str(),
        ))
        .target("aarch64-linux-android")
        .file("src\\misc.c")
        .include(Path::new(format!("{}/lumin/usr/include", mlsdk).as_str()))
        .compile("misc");
}
