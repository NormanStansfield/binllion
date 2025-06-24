pub(crate) trait AppTrait: Drop {
    fn run(&mut self);
    fn new() -> Self;
    fn is_running(&self) -> bool;
    fn quit(&mut self);
}

pub(crate) trait NoticeProviderTrait {
    fn new() -> Self;
    fn add(&mut self, notice: Notice);
    fn get_notice(&mut self) -> Notice;
}

use nutype::nutype;

use crate::write_mode::WriteMode;
#[nutype(sanitize(trim), derive(Default), default = "")]
pub(crate) struct Notice(String);

#[nutype(derive(Clone))]
pub(crate) struct FilePath(Option<std::ffi::OsString>);

trait CurrentFileTrait {
    fn new() -> Self;
    fn get_path() -> FilePath;
    fn set_path(path: FilePath);
}

pub(crate) trait BinDataTrait {
    fn new() -> Self;
    fn add_data(&mut self, value: HexData);
    fn insert_data(&mut self, index: Index, value: HexData);
    fn delete_data(&mut self, index: Index);
    fn update_data(&mut self, index: Index, value: HexData);
    fn import_from(&mut self, path: FilePath) -> std::io::Result<()>;
    fn export_to(&self) -> std::io::Result<()>;
    fn get_data(&self, index: Index) -> Result<HexData, ()>;
    fn get_file_name(&self) -> Notice;
    fn get_size(&mut self) -> Index;
    fn get_slice(&mut self) -> Vec<u8>;
}

#[nutype(derive(Clone, AsRef))]
pub(crate) struct HexData(u8);

#[nutype(derive(Clone, AsRef))]
pub(crate) struct Index(usize);

// pub(crate) struct BinData {
//     buf: std::collections::VecDeque<u8>,
// }

pub(crate) trait KeyEventHandlerTrait {
    // fn new() -> Self;
    fn handle_key_event() -> Command;
}

#[derive(Debug)]
pub(crate) enum Command {
    MoveToUp,
    MoveToDown,
    MoveToLeft,
    MoveToRight,
    ChangeWriteMode,
    ImportFile,
    ExportFile,
    Exit,
    DeleteData,
    Nop,
    InputData(CharCode),
}

pub(crate) trait MiniBufTrait {
    fn new() -> Self;
    // fn reset_buf(&mut self);
    fn add(&mut self, char_code: CharCode) -> MiniBufPosition;
    fn updata(&mut self, value: HexData);
    fn to_hex(&self) -> Result<HexData, std::num::ParseIntError>;
    fn get_position(&self) -> MiniBufPosition;
}

#[derive(PartialEq)]
pub(crate) enum MiniBufPosition {
    Head,
    Tail,
}

// #[nutype()]
#[nutype(sanitize(with = |char| char.to_ascii_lowercase()), derive(AsRef, Debug))]
pub(crate) struct CharCode(char);

pub(crate) trait BinDataIndexTrait {
    fn new() -> Self;
    fn index(&self) -> Index;
    fn move_to_right(&mut self) -> Index;
    fn move_to_left(&mut self) -> Index;
    fn move_to_up(&mut self) -> Index;
    fn move_to_down(&mut self) -> Index;
    fn reset_index(&mut self);
    fn set_size(&mut self, value: Index);
}

trait CursorPositionTrait {
    fn show_cursor(&self, terminal: &mut ratatui::DefaultTerminal);
}
struct CursorPosition {
    // const STEP: usize = 3,
    // const ORIGIN_X: u16 = 10,
    // const ORIGIN_Y: u16 = 2,
    position: ratatui::prelude::Position,
}

pub(crate) trait WriteModeTrait {
    // fn toggle_mode(&mut self);
    fn toggle_mode(&self) -> WriteMode;
}

pub(crate) trait TuiLayoutProviderTrait {
    fn get_layout(terminal: &mut ratatui::DefaultTerminal) -> TuiLayout;
}

pub(crate) struct TuiLayout {
    pub(crate) main_panel: TuiArea,
    pub(crate) main_header: TuiArea,
    pub(crate) main_content: TuiArea,
    pub(crate) ascii_panel: TuiArea,
    pub(crate) ascii_header: TuiArea,
    pub(crate) ascii_content: TuiArea,
    pub(crate) versatile: TuiArea,
}

#[nutype(derive(Clone, AsRef))]
pub(crate) struct TuiArea(ratatui::prelude::Rect);

pub(crate) trait TuiPanelCommonTrait {
    fn new() -> Self;
    // fn set_layout(&mut self, layout: TuiLayout);
    // fn set_layout(&mut self, layout: TuiArea);
    // fn draw(&mut self, terminal: &mut ratatui::DefaultTerminal);
}

pub(crate) trait TuiMainPanelTrait {
    fn set_title(&mut self, title: Notice);
    fn set_err_msg(&mut self, message: Notice);
    fn set_mode(&mut self, mode: WriteMode);
    // fn set_content(&mut self, bin_data: Vec<u8>);
}

pub(crate) trait TuiMainContentTrait {
    fn set_content(&mut self, bin_data: Vec<u8>);
}

// pub(crate) struct ErrorMessage(String);

pub(crate) trait TuiAsciiPanelTrait {
    fn set_content(bin_data: &[u8]);
}

pub(crate) trait TuiVersatilePanelTrait {}
