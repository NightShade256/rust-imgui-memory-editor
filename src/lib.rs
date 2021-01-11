//! Safe Rust bindings to Dear ImGui memory editor widget.
//!  
//! See [`MemoryEditor`] struct for more.

mod memory_editor;

// Rexport safe wrapper.
pub use crate::memory_editor::MemoryEditor;
