use imgui::{ImColor, ImStr, Ui};
use imgui_memory_editor_sys as sys;

/// Dear ImGui memory editor widget.
#[derive(Debug)]
pub struct MemoryEditor {
    raw_editor: sys::MemoryEditor,
}

impl Default for MemoryEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryEditor {
    /// Create a new `MemoryEditor` instance.
    pub fn new() -> Self {
        Self {
            raw_editor: sys::MemoryEditor {
                // Settings
                Open: false,
                ReadOnly: false,
                Cols: 16,
                OptShowOptions: true,
                OptShowDataPreview: false,
                OptShowHexII: false,
                OptShowAscii: true,
                OptGreyOutZeroes: true,
                OptUpperCaseHex: true,
                OptMidColsCount: 8,
                OptAddrDigitsCount: 0,
                HighlightColor: 855638015, // (255, 255, 255, 50)
                ReadFn: None,
                WriteFn: None,
                HighlightFn: None,

                // State/Internals
                ContentsWidthChanged: false,
                DataPreviewAddr: usize::MAX - 1,
                DataEditingAddr: usize::MAX - 1,
                DataEditingTakeFocus: false,
                DataInputBuf: [0; 32],
                AddrInputBuf: [0; 32],
                GotoAddr: usize::MAX - 1,
                HighlightMin: usize::MAX - 1,
                HighlightMax: usize::MAX - 1,
                PreviewEndianess: 0,
                PreviewDataType: 4, // ImGuiDataType_S32
            },
        }
    }

    /// Get the `ReadOnly` field.
    pub fn get_read_only(&self) -> bool {
        self.raw_editor.ReadOnly
    }

    /// Set the `ReadOnly` field.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.raw_editor.ReadOnly = read_only;
    }

    /// Get the `Cols` field.
    pub fn get_cols(&self) -> i32 {
        self.raw_editor.Cols
    }

    /// Set the `Cols` field.
    pub fn set_cols(&mut self, columns: i32) {
        self.raw_editor.Cols = columns;
    }

    /// Get the `HighlightColor` field.
    pub fn get_highlight_color(&self) -> ImColor {
        self.raw_editor.HighlightColor.into()
    }

    /// Set the `HighlightColor` field.
    pub fn set_highlight_color(&mut self, color: ImColor) {
        self.raw_editor.HighlightColor = color.into()
    }

    /// Get the `OptShowHexII` field.
    pub fn get_show_hexii(&self) -> bool {
        self.raw_editor.OptShowHexII
    }

    /// Set the `OptShowHexII` field.
    pub fn set_show_hexii(&mut self, read_only: bool) {
        self.raw_editor.OptShowHexII = read_only;
    }

    /// Get the `OptShowAscii` field.
    pub fn get_show_ascii(&self) -> bool {
        self.raw_editor.OptShowAscii
    }

    /// Set the `OptShowAscii` field.
    pub fn set_show_ascii(&mut self, read_only: bool) {
        self.raw_editor.OptShowAscii = read_only;
    }

    // Render memory editor contents only.
    pub fn draw_contents(&mut self, _: &Ui, mem_data: &mut [u8], base_display_addr: Option<usize>) {
        let mem_edit = (&mut self.raw_editor) as *mut sys::MemoryEditor;

        let mem_size = mem_data.len();
        let mem_data = mem_data.as_mut_ptr() as *mut cty::c_void;

        unsafe {
            sys::DrawContents(
                mem_edit,
                mem_data,
                mem_size,
                base_display_addr.unwrap_or(0x0000),
            );
        }
    }

    // Render standalone memory editor window.
    pub fn draw_window(
        &mut self,
        _: &Ui,
        title: &ImStr,
        mem_data: &mut [u8],
        base_display_addr: Option<usize>,
    ) {
        let mem_edit = (&mut self.raw_editor) as *mut sys::MemoryEditor;

        let mem_size = mem_data.len();
        let mem_data = mem_data.as_mut_ptr() as *mut cty::c_void;

        unsafe {
            sys::DrawWindow(
                mem_edit,
                title.as_ptr(),
                mem_data,
                mem_size,
                base_display_addr.unwrap_or(0x0000),
            );
        }
    }
}
