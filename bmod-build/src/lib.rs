//! BakkesModSDK for Rust build-script helper.
//!
//! This crate builds the C++ boilerplate part of the plugin.

pub use bmod;
use bmod::PluginType;
use cxx_gen::{Include, IncludeKind};
use quote::quote;
use std::path::PathBuf;

/// Compile the SDK into a plugin facade with bidirectional FFI support.
///
/// # Arguments
///
/// All arguments will be visible in the bakkesmod UI.
///
/// - `crate_name`: The plugin name.
/// - `crate_version`: The plugin version.
/// - `class_name`: Name of the C++ class in the boilerplate.
/// - `flags`: The [plugin type](https://wiki.bakkesplugins.com/code_snippets/plugin_types/).
pub fn compile(crate_name: &str, crate_version: &str, class_name: &str, flags: PluginType) {
    let crate_name = crate_name.trim();
    let crate_version = crate_version.trim();
    let class_name = class_name.trim();

    assert_ne!(crate_name, "");
    assert_ne!(crate_version, "");
    assert_ne!(class_name, "");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bridge_path = PathBuf::from_iter([&out_dir, "plugin.rs"]);
    let header_path = PathBuf::from_iter([&out_dir, "plugin.h"]);
    let cpp_path = PathBuf::from_iter([&out_dir, "plugin.cc"]);
    let gen_h_path = PathBuf::from_iter([&out_dir, "gen.h"]);
    let gen_cpp_path = PathBuf::from_iter([&out_dir, "gen.cc"]);
    let cxx_header_path = PathBuf::from_iter([&out_dir, "rust", "cxx.h"]);
    let bakkesmod_inc_path =
        PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "BakkesModSDK", "include"]);
    let bakkesmod_lib_path =
        PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "BakkesModSDK", "lib"]);

    // Create the OUT_DIR directory structure.
    std::fs::create_dir_all(cxx_header_path.parent().unwrap()).unwrap();

    // Write Rust boilerplate to OUT_DIR.
    let bridge = quote! {
        #[cxx::bridge]
        mod ffi {
            extern "Rust" {
                fn on_load();
                fn on_unload();
            }

            unsafe extern "C++" {
                fn console_log(msg: &str);
            }
        }
    };
    std::fs::write(bridge_path, bridge.to_string())
        .expect("Unable to write cxx::bridge boilerplate");

    // Write C++ header to OUT_DIR.
    let header = format!(
        include_str!("./templates/plugin.h.tmpl"),
        cxx_header_path = cxx_header_path.display(),
        class_name = class_name,
    );
    std::fs::write(&header_path, header).expect("Unable to write C++ header");

    // Write C++ boilerplate to OUT_DIR.
    let cpp = format!(
        include_str!("./templates/plugin.cc.tmpl"),
        header_path = header_path.display(),
        gen_h_path = gen_h_path.display(),
        class_name = class_name,
        crate_name = crate_name,
        crate_version = crate_version,
        flags = flags.to_string(),
    );
    std::fs::write(&cpp_path, cpp).expect("Unable to write C++ boilerplate");

    let mut options = cxx_gen::Opt::default();
    options.include.push(Include {
        path: header_path.display().to_string(),
        kind: IncludeKind::Quoted,
    });
    let gen = cxx_gen::generate_header_and_cc(bridge, &options).unwrap();

    // Write generated C++ header to OUT_DIR.
    std::fs::write(&gen_h_path, gen.header).unwrap();

    // Write generated C++ implementation to OUT_DIR.
    std::fs::write(&gen_cpp_path, gen.implementation).unwrap();

    // Write cxx header to OUT_DIR.
    std::fs::write(&cxx_header_path, cxx_gen::HEADER).unwrap();

    // Build the C++ library.
    cc::Build::new()
        .cpp(true)
        .file(cpp_path)
        .file(gen_cpp_path)
        .include(bakkesmod_inc_path)
        .include(out_dir)
        .compile(crate_name);

    // Only rebuild when the build script changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Set link args.
    println!("cargo:rustc-link-search={}", bakkesmod_lib_path.display());
    println!("cargo:rustc-link-lib=pluginsdk");
    println!("cargo:rustc-link-arg=/WHOLEARCHIVE:{crate_name}.lib");
}
