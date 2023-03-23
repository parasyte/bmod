# `bmod`

Experimental Rust bindings for the C++ [BakkesModSDK](https://github.com/bakkesmodorg/BakkesModSDK), a plugin API for Rocket League.


## WARNING: Here be dragons

This project is currently in an experimental state and should not be used for production code. Most functionality is unimplemented and there may be bugs, potentially including Undefined Behavior.

You are highly encouraged to use the C++ SDK today.


## Using `bmod` anyway

Still interested? The TL;DR quick-start is:

```bash
$ cargo install-plugin --release -p test-plugin
```

This will build (in release mode) the [`test-plugin`](./examples/test-plugin/src/main.rs) crate and attempt to install it to the default plugin directory. It also adds itself to

The test plugin only prints a short message to the F6 console when loaded, and another message when unloaded.


## Architecture

The bindings are created with the venerable [`cxx`](https://docs.rs/cxx) crate and its siblings. There are two moving parts:

- `bmod` provides types, macros, and utilities for interacting with the SDK API.
- `bmod-build` produces the bindings. Intended to be used in build-scripts.

`bmod_build::compile()` is a code generator. It outputs several source files (C++ and Rust), builds the SDK as a library, and configures Cargo to link it properly.

`bmod::plugin! {}` is the main entry point. It requires defining an `on_load()` function, and optionally allows an `on_unload()` function. These will be called automatically by bakkesmod when the plugin is loaded and unloaded, respectively.

`on_load` and `on_unload` accept no arguments and return no values. All SDK interactions are done through the static `singleton` reference. This is also true for macros like `bmod::console_log!()`. The singleton reference is never exposed directly to Rust, but is implicitly referenced through exported C++ functions.


## Testing

To run the tests, make sure the `bakkesmod/dll/` directory is in your path:

```
PATH="$APPDATA/bakkesmod/bakkesmod/dll:$PATH" cargo test --workspace
```

If the path environment is configured incorrectly, you will get an error that looks like this:

```
error: test failed, to rerun pass `-p test-plugin --lib`

Caused by:
  process didn't exit successfully: `C:\Users\jay\projects\bmod\target\debug\deps\test_plugin-0768ef7d8061cc2a.exe` (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
C:/Users/jay/.cargo/bin/cargo.exe: error while loading shared libraries: ?: cannot open shared object file: No such file or directory
```


## TODO

- Thread safety: Create a Rust-side thread-local representation of the singleton with borrow guarantees and ensure it is `!Send`.
- Implicit singleton access in exported C++ functions is unsound.
  - Require a reference as an argument. This will prevent Rust from invoking UB by accessing the singleton implicitly from multiple threads through exported function calls.
- Add all of the wrapper types (there are a ton of them).
  - Or at least the most useful ones. The deprecated types and methods are not necessary.
- `imgui-sys` bindings and a safe interface for it.
  - The safe `imgui` crate cannot be used because it doesn't allow setting arbitrary context pointers (that would be unsafe!)
