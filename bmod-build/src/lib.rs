//! BakkesModSDK for Rust build-script helper.
//!
//! This crate builds the C++ boilerplate part of the plugin.

use bmod::PluginType;
use cxx_gen::{Include, IncludeKind};
use quote::quote;

pub fn compile(crate_name: &str, crate_version: &str, class_name: &str, flags: PluginType) {
    let crate_name = crate_name.trim();
    let crate_version = crate_version.trim();
    let class_name = class_name.trim();

    assert_ne!(crate_name, "");
    assert_ne!(crate_version, "");
    assert_ne!(class_name, "");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bridge_path = format!("{out_dir}/plugin.rs");
    let header_path = format!("{out_dir}/plugin.h");
    let cpp_path = format!("{out_dir}/plugin.cc");
    let gen_h_path = format!("{out_dir}/gen.h");
    let gen_cpp_path = format!("{out_dir}/gen.cc");
    let cxx_header_path = format!("{out_dir}/rust/cxx.h");

    std::fs::create_dir_all(format!("{out_dir}/rust")).unwrap();

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

    // Write Rust boilerplate to OUT_DIR.
    std::fs::write(bridge_path, bridge.to_string())
        .expect("Unable to write cxx::bridge boilerplate");

    let header = format!(
        r#"
#pragma once
#include "{cxx_header_path}"
#include "bakkesmod/plugin/bakkesmodplugin.h"

class {class_name} : public BakkesMod::Plugin::BakkesModPlugin {{
public:
    virtual void onLoad();
    virtual void onUnload();
}};

void console_log(rust::Str msg);
"#
    );
    // Write C++ header to OUT_DIR.
    std::fs::write(&header_path, header).expect("Unable to write C++ header");

    let cpp = format!(
        r#"
#include "{header_path}"
#include "bakkesmod/wrappers/includes.h"
#include "{gen_h_path}"

BAKKESMOD_PLUGIN({class_name}, "{crate_name}", "{crate_version}", {flags})

void {class_name}::onLoad() {{
    on_load();
}}

void {class_name}::onUnload() {{
    on_unload();
}}

void console_log(rust::Str msg) {{
    singleton->cvarManager->log(std::string(msg));
}}
"#,
        flags = flags.to_string(),
    );
    // Write C++ boilerplate to OUT_DIR.
    std::fs::write(&cpp_path, cpp).expect("Unable to write C++ boilerplate");

    let mut options = cxx_gen::Opt::default();
    options.include.push(Include {
        path: header_path,
        kind: IncludeKind::Quoted,
    });
    let gen = cxx_gen::generate_header_and_cc(bridge, &options).unwrap();

    // Write generated C++ header to OUT_DIR.
    std::fs::write(&gen_h_path, gen.header).unwrap();

    // Write generated C++ implementation to OUT_DIR.
    std::fs::write(&gen_cpp_path, gen.implementation).unwrap();

    // Write cxx header to OUT_DIR.
    std::fs::write(&cxx_header_path, cxx_gen::HEADER).unwrap();

    cc::Build::new()
        .cpp(true)
        .file(cpp_path)
        .file(gen_cpp_path)
        // TODO: How to get the SDK path for crates outside of the workspace?
        .include("../../BakkesModSDK/include")
        .include(out_dir)
        .compile(crate_name);
}
