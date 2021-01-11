//! Compiles memory editor widget before the crate
//! is compiled.

fn main() {
    let mut build = cc::Build::default();

    build
        .cpp(true)
        .define("IMGUI_DISABLE_OSX_FUNCTIONS", None)
        .define("IMGUI_DISABLE_WIN32_FUNCTIONS", None)
        .warnings(false)
        .file("wrapper.cpp")
        .compile("imgui_memory_editor");
}
