//! Safe Rust bindings to Dear ImGui memory editor widget.

mod memory_editor;

// Rexport safe wrapper.
pub use crate::memory_editor::MemoryEditor;
