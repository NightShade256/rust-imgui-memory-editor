/* automatically generated by rust-bindgen 0.56.0 */

#![allow(nonstandard_style, clippy::all)]

pub type ImGuiDataType = cty::c_int;
pub type ImU8 = cty::c_uchar;
pub type ImU32 = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryEditor {
    pub Open: bool,
    pub ReadOnly: bool,
    pub Cols: cty::c_int,
    pub OptShowOptions: bool,
    pub OptShowDataPreview: bool,
    pub OptShowHexII: bool,
    pub OptShowAscii: bool,
    pub OptGreyOutZeroes: bool,
    pub OptUpperCaseHex: bool,
    pub OptMidColsCount: cty::c_int,
    pub OptAddrDigitsCount: cty::c_int,
    pub HighlightColor: ImU32,
    pub ReadFn: ::core::option::Option<unsafe extern "C" fn(data: *const ImU8, off: usize) -> ImU8>,
    pub WriteFn: ::core::option::Option<unsafe extern "C" fn(data: *mut ImU8, off: usize, d: ImU8)>,
    pub HighlightFn:
        ::core::option::Option<unsafe extern "C" fn(data: *const ImU8, off: usize) -> bool>,
    pub ContentsWidthChanged: bool,
    pub DataPreviewAddr: usize,
    pub DataEditingAddr: usize,
    pub DataEditingTakeFocus: bool,
    pub DataInputBuf: [cty::c_char; 32usize],
    pub AddrInputBuf: [cty::c_char; 32usize],
    pub GotoAddr: usize,
    pub HighlightMin: usize,
    pub HighlightMax: usize,
    pub PreviewEndianess: cty::c_int,
    pub PreviewDataType: ImGuiDataType,
}
pub const MemoryEditor_DataFormat_DataFormat_Bin: MemoryEditor_DataFormat = 0;
pub const MemoryEditor_DataFormat_DataFormat_Dec: MemoryEditor_DataFormat = 1;
pub const MemoryEditor_DataFormat_DataFormat_Hex: MemoryEditor_DataFormat = 2;
pub const MemoryEditor_DataFormat_DataFormat_COUNT: MemoryEditor_DataFormat = 3;
pub type MemoryEditor_DataFormat = cty::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryEditor_Sizes {
    pub AddrDigitsCount: cty::c_int,
    pub LineHeight: f32,
    pub GlyphWidth: f32,
    pub HexCellWidth: f32,
    pub SpacingBetweenMidCols: f32,
    pub PosHexStart: f32,
    pub PosHexEnd: f32,
    pub PosAsciiStart: f32,
    pub PosAsciiEnd: f32,
    pub WindowWidth: f32,
}
extern "C" {
    #[link_name = "\u{1}?DrawContents@@YAXPEAUMemoryEditor@@PEAX_K2@Z"]
    pub fn DrawContents(
        mem_edit: *mut MemoryEditor,
        mem_data: *mut cty::c_void,
        mem_size: usize,
        base_display_addr: usize,
    );
}
extern "C" {
    #[link_name = "\u{1}?DrawWindow@@YAXPEAUMemoryEditor@@PEBDPEAX_K3@Z"]
    pub fn DrawWindow(
        mem_edit: *mut MemoryEditor,
        title: *const cty::c_char,
        mem_data: *mut cty::c_void,
        mem_size: usize,
        base_display_addr: usize,
    );
}
