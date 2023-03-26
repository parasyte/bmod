//! BakkesModSDK for Rust

use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PluginType: u32 {
        const ALL = 0x00;
        const FREEPLAY = 0x01;
        const CUSTOM_TRAINING = 0x02;
        const SPECTATOR = 0x04;
        const BOTAI = 0x08;
        const REPLAY = 0x10;
        const THREADED = 0x20;
        const THREADEDUNLOAD = 0x40;
    }
}

impl ToString for PluginType {
    fn to_string(&self) -> String {
        let names = self
            .iter_names()
            .filter_map(|(name, _)| (name != "ALL").then_some(name));

        let mut flags = String::new();
        for name in names {
            if !flags.is_empty() {
                flags.push('|');
            }
            flags.push_str("PLUGINTYPE_");
            flags.push_str(name);
        }

        if flags.is_empty() {
            "0".to_string()
        } else {
            flags
        }
    }
}

#[macro_export]
macro_rules! plugin {
    (fn on_load() $on_load_body:block) => {
        bmod_build::plugin! {
            fn on_load() $on_load_body
            fn on_unload() {}
        }
    };

    (fn on_load() $on_load_body:block fn on_unload() $on_unload_body:block) => {
        include!(concat!(env!("OUT_DIR"), "/plugin.rs"));

        fn on_load() $on_load_body
        fn on_unload() $on_unload_body
    };
}

/// Log a formatted string to the BakkesMod F6 console.
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! console_log {
    ($($args:tt)*) => {
        cxx::let_cxx_string!(msg = "");
        std::fmt::Write::write_fmt(&mut msg.as_mut(), format_args!($($args),*)).unwrap();

        crate::ffi::console_log(&msg);
    }
}
