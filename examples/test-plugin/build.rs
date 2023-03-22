fn main() {
    bmod_build::compile(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        "TestPlugin",
        bmod::PluginType::ALL,
    );
}
