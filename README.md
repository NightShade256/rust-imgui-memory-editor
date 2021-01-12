# Dear ImGui Memory Editor

Safe Rust bindings to the Dear ImGui memory editor widget, to be used with `imgui-rs`.

## Note

This crate is *not* published on `crates.io` since there is already
a crate on `crates.io` which implements bindings to the widget with the same name.

The crate on `crates.io` is outdated, and has a totally different API while
this crate tries to stay close to the original widget's API.

## Usage

You can use this crate by adding the following in your `Cargo.toml`

```toml
[dependencies]
imgui-memory-editor = { git = "https://github.com/NightShade256/rust-imgui-memory-editor" }
```

The `master` branch will only be pushed on a new release, and further work will
occur in the `development` branch.

## Documentation

You can access the docs for this crate [here](https://nightshade256.github.io/rust-imgui-memory-editor/imgui_memory_editor/index.html).

## License

Both the sys and wrapper crates are licensed under the MIT license.
