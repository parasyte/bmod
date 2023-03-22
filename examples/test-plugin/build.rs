// use std::path::PathBuf;

fn main() {
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    // // TODO: Move this stuff to bmod-build and figure out the right way to query the build-time env
    // let out_dir = std::env::var("OUT_DIR").unwrap();
    // let mut lib_path = PathBuf::from(&out_dir);
    // lib_path.push("/BakkesModSDK/lib");
    // // let lib_path = format!("{out_dir}/BakkesModSDK/lib");

    bmod_build::compile(
        crate_name,
        crate_version,
        "TestPlugin",
        bmod::PluginType::ALL,
    );

    // // Only rebuild when the build script changes.
    // println!("cargo:rerun-if-changed=build.rs");

    // println!("cargo:rustc-link-search={}", lib_path.display());
    // println!("cargo:rustc-link-lib=pluginsdk");
    // println!("cargo:rustc-link-arg=/WHOLEARCHIVE:{crate_name}.lib");
}
