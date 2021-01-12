#include "third-party/imgui/imgui.h"
#include "third-party/imgui/imgui_internal.h"
#include "third-party/imgui_club/imgui_memory_editor/imgui_memory_editor.h"

extern "C" {
// Wrapper around MemoryEditor.DrawContents
void DrawContents(
    MemoryEditor* mem_edit, void* mem_data, size_t mem_size, size_t base_display_addr) {
    mem_edit->DrawContents(mem_data, mem_size, base_display_addr);
}

// Wrapper around MemoryEditor.DrawWindow
void DrawWindow(MemoryEditor* mem_edit, const char* title, void* mem_data,
    size_t mem_size, size_t base_display_addr) {
    mem_edit->DrawWindow(title, mem_data, mem_size, base_display_addr);
}
}
