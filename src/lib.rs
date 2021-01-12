//! Safe Rust bindings to Dear ImGui memory editor widget.
//!  
//! See [`MemoryEditor`] struct for more.

mod memory_editor;

// Export safe wrapper.
pub use crate::memory_editor::MemoryEditor;

// Re-export ImU8 type.
pub use imgui_memory_editor_sys::ImU8;
