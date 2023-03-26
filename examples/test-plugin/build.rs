fn main() {
    bmod_build::compile(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        "TestPlugin",
        bmod_build::bmod::PluginType::ALL,
    );

    // Set PATH environment variable to allow `cargo test` to work.
    let appdata = std::env::var("APPDATA").unwrap();
    let path = std::env::var("PATH").unwrap();
    println!("cargo:rustc-env=PATH={appdata}/bakkesmod/bakkesmod/dll;{path}");
}
