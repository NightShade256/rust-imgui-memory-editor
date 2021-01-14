use std::marker::PhantomData;
use std::mem;

use imgui::{ImColor, ImStr, Ui};
use imgui_memory_editor_sys as sys;

/// Dear ImGui memory editor widget.
///  
/// Use by calling either `draw_contents` or
/// `draw_window` methods.
///  
/// ## Note:
/// While theoretically this struct is Send + Sync compatible, it doesn't
/// implement those traits, since Dear ImGui
/// [isn't thread safe](https://github.com/imgui-rs/imgui-rs/issues/392#issuecomment-737779381).
#[derive(Debug)]
pub struct MemoryEditor {
    raw_editor: sys::MemoryEditor,

    /// Avoid implementing Send and Sync, since Dear ImGui
    /// is not thread safe.
    /// [https://github.com/imgui-rs/imgui-rs/issues/392#issuecomment-737779381]
    _phantom_data: PhantomData<*const ()>,
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
            _phantom_data: PhantomData,
        }
    }

    /// Get the `Open` field.
    pub fn get_open(&self) -> bool {
        self.raw_editor.Open
    }

    /// Set the `Open` field.
    pub fn set_open(&mut self, open: bool) {
        self.raw_editor.Open = open;
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
    pub fn get_cols(&self) -> cty::c_int {
        self.raw_editor.Cols
    }

    /// Set the `Cols` field.
    pub fn set_cols(&mut self, columns: cty::c_int) {
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
    pub fn set_show_hexii(&mut self, show: bool) {
        self.raw_editor.OptShowHexII = show;
    }

    /// Get the `OptShowAscii` field.
    pub fn get_show_ascii(&self) -> bool {
        self.raw_editor.OptShowAscii
    }

    /// Set the `OptShowAscii` field.
    pub fn set_show_ascii(&mut self, show: bool) {
        self.raw_editor.OptShowAscii = show;
    }

    /// Get the `OptShowOptions` field.
    pub fn get_show_options(&self) -> bool {
        self.raw_editor.OptShowOptions
    }

    /// Set the `OptShowOptions` field.
    pub fn set_show_options(&mut self, show: bool) {
        self.raw_editor.OptShowOptions = show;
    }

    /// Get the `OptShowDataPreview` field.
    pub fn get_show_data_preview(&self) -> bool {
        self.raw_editor.OptShowDataPreview
    }

    /// Set the `OptShowDataPreview` field.
    pub fn set_show_data_preview(&mut self, show: bool) {
        self.raw_editor.OptShowDataPreview = show;
    }

    /// Get the `OptGreyOutZeroes` field.
    pub fn get_grey_out_zeroes(&self) -> bool {
        self.raw_editor.OptGreyOutZeroes
    }

    /// Set the `OptGreyOutZeroes` field.
    pub fn set_grey_out_zeroes(&mut self, greyout: bool) {
        self.raw_editor.OptGreyOutZeroes = greyout;
    }

    /// Get the `OptUpperCaseHex` field.
    pub fn get_upper_case_hex(&self) -> bool {
        self.raw_editor.OptUpperCaseHex
    }

    /// Set the `OptUpperCaseHex` field.
    pub fn set_upper_case_hex(&mut self, uppercase: bool) {
        self.raw_editor.OptUpperCaseHex = uppercase;
    }

    /// Get the `OptMidColsCount` field.
    pub fn get_mid_cols_count(&self) -> cty::c_int {
        self.raw_editor.OptMidColsCount
    }

    /// Set the `OptMidColsCount` field.
    pub fn set_mid_cols_count(&mut self, count: cty::c_int) {
        self.raw_editor.OptMidColsCount = count;
    }

    /// Get the `OptAddrDigitsCount` field.
    pub fn get_addr_digits_count(&self) -> cty::c_int {
        self.raw_editor.OptAddrDigitsCount
    }

    /// Set the `OptAddrDigitsCount` field.
    pub fn set_addr_digits_count(&mut self, count: cty::c_int) {
        self.raw_editor.OptAddrDigitsCount = count;
    }

    /// Set the `ReadFn` field.
    ///  
    /// You can only pass in ordinary functions
    /// and stateless closures, if you pass anything
    /// else this function **will panic**.
    pub fn set_read_fn<F>(&mut self, _: F)
    where
        F: Fn(*const sys::ImU8, usize) -> sys::ImU8,
    {
        assert!(mem::size_of::<F>() == 0);

        unsafe extern "C" fn wrapped<F: Fn(*const sys::ImU8, usize) -> sys::ImU8>(
            mem_data: *const sys::ImU8,
            offset: usize,
        ) -> sys::ImU8 {
            mem::zeroed::<F>()(mem_data, offset)
        }

        self.raw_editor.ReadFn = Some(wrapped::<F>);
    }

    /// Set the `WriteFn` field.
    ///  
    /// You can only pass in ordinary functions
    /// and stateless closures, if you pass anything
    /// else this function **will panic**.
    pub fn set_write_fn<F>(&mut self, _: F)
    where
        F: Fn(*mut sys::ImU8, usize, sys::ImU8),
    {
        assert!(mem::size_of::<F>() == 0);

        unsafe extern "C" fn wrapped<F: Fn(*mut sys::ImU8, usize, sys::ImU8)>(
            mem_data: *mut sys::ImU8,
            offset: usize,
            value: sys::ImU8,
        ) {
            mem::zeroed::<F>()(mem_data, offset, value);
        }

        self.raw_editor.WriteFn = Some(wrapped::<F>);
    }

    /// Set the `HighlightFn` field.
    ///  
    /// You can only pass in ordinary functions
    /// and stateless closures, if you pass anything
    /// else this function **will panic**.
    pub fn set_highlight_fn<F>(&mut self, _: F)
    where
        F: Fn(*const sys::ImU8, usize) -> bool,
    {
        assert!(mem::size_of::<F>() == 0);

        unsafe extern "C" fn wrapped<F: Fn(*const sys::ImU8, usize) -> bool>(
            mem_data: *const sys::ImU8,
            offset: usize,
        ) -> bool {
            mem::zeroed::<F>()(mem_data, offset)
        }

        self.raw_editor.HighlightFn = Some(wrapped::<F>);
    }

    /// Render only the contents of the memory editor, without any window.
    ///  
    /// The `base_display_addr` field has a default value of `0x0000` if
    /// `None` is passed as an argument.
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

    /// Render standalone memory editor, in a window.
    ///  
    /// The `base_display_addr` field has a default value of `0x0000` if
    /// `None` is passed as an argument.
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
