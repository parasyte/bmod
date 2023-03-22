fn main() {
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");
    // TODO: How to get the SDK path for crates outside of the workspace?
    let lib_path = "./BakkesModSDK/lib";

    bmod_build::compile(
        crate_name,
        crate_version,
        "TestPlugin",
        bmod::PluginType::ALL,
    );

    // Only rebuild when the build script changes.
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-link-search={lib_path}");
    println!("cargo:rustc-link-lib=pluginsdk");
    println!("cargo:rustc-link-arg=/WHOLEARCHIVE:{crate_name}.lib");
}
