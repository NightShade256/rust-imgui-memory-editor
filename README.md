# Dear ImGui Memory Editor

Safe, Rust bindings to the Dear ImGui memory editor widget, to be used with `imgui-rs`.

## Note

Not to be confused with the crate on `crates.io` with the same name.

This crate is a completely different crate from the one on `crates.io` but both
try to achieve the same goal, hence you can use whichever you want.

This crate tries to be as similar as possible to the C++ widget, while the one
on `crates.io` has a bit of a different API.

## Usage

Since the crate is not available on `crates.io`, you cannot directly specify the version
and instead must add the following to your `Cargo.toml`,

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
