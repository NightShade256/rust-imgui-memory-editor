//! Raw, unsafe no_std bindings to Dear ImGui memory editor widget.

#![no_std]

mod bindings;

// Re-export all automatically generated bindings.
pub use crate::bindings::*;
