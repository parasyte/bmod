use bmod::console_log;

bmod::plugin! {
    fn on_load() {
        let life = 42;
        console_log!("Hello, {life}");
    }

    fn on_unload() {
        crate::ffi::console_log("Goodbye!");
    }
}
