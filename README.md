# Dear ImGui Memory Editor

Safe, Rust bindings to the Dear ImGui memory editor widget, to be used with `imgui-rs`.

## Note

These bindings were made for personal use, and are limited in use.

Please checkout [imgui-memory-editor](https://crates.io/crates/imgui-memory-editor) crate on `crates.io` which is a more
comprehensive set of bindings for the same.

This crate tries to be minimal, and close to the original C++ API.

## Usage

Since the crate is not available on `crates.io`, you have to add the following to your `Cargo.toml`,

```toml
[dependencies]
imgui-memory-editor = { git = "https://github.com/NightShade256/rust-imgui-memory-editor" }
```

The `master` branch will only be pushed on a new release, and further work will
occur in the `development` branch.

## Documentation

You can access the docs for this crate [here](https://nightshade256.github.io/rust-imgui-memory-editor/imgui_memory_editor/index.html).

## License

This project is licensed under the terms of the MIT license.
