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

use std::ffi::OsString;

use nutype::nutype;

use crate::write_mode::WriteMode;
#[nutype(sanitize(trim), derive(Default), default = "")]
pub(crate) struct Notice(String);

#[nutype()]
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
    fn import_from(&mut self, path: FilePath) -> Result<(), std::io::Error>;
    fn export_to(&self, path: FilePath) -> Result<(), std::io::Error>;
    fn data_from_index(&mut self, index: Index) -> Result<HexData, ()>;
    fn get_file_name(&self) -> Notice;
}

pub(crate) struct HexData(pub u8);

pub(crate) struct Index(pub usize);

pub(crate) struct BinData {
    buf: std::collections::VecDeque<u8>,
}

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
    Nope,
    InputData(CharCode),
}

trait MiniBufTrait {
    fn new() -> Self;
    // fn reset_buf(&mut self);
    fn add(&mut self, char_code: CharCode);
    fn data_from_index(&mut self, index: Index);
    fn to_hex(&self) -> HexData;
}

// #[nutype()]
#[nutype(sanitize(with = |char| char.to_ascii_lowercase()), derive(AsRef, Debug))]
pub(crate) struct CharCode(char);

pub(crate) struct MiniBuf {
    buf: [char; 2],
    index: Index,
}

trait BinDataIndexTrait {
    fn new() -> Self;
    fn index(&self) -> Self;
    fn move_to_right(&mut self) -> Self;
    fn move_to_left(&mut self) -> Self;
    fn move_to_up(&mut self) -> Self;
    fn move_to_down(&mut self) -> Self;
    fn reset_index(&mut self);
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

trait TuiLayoutProvider {
    fn get_layout() -> TuiLayout;
}

struct TuiLayout {
    main_panel: TuiArea,
    main_header: TuiArea,
    main_content: TuiArea,
    ascii_panel: TuiArea,
    ascii_header: TuiArea,
    ascii_content: TuiArea,
    versatile: TuiArea,
}

pub(crate) struct TuiArea(ratatui::prelude::Rect);

pub(crate) trait TuiPanelCommonTrait {
    fn new() -> Self;
    fn set_layout(layout: TuiArea);
    fn draw(&mut self, terminal: &mut ratatui::DefaultTerminal);
}

pub(crate) trait TuiMainPanelTrait {
    fn set_title(&mut self, title: Notice);
    fn set_err_msg(&mut self, message: Notice);
    fn set_mode(&mut self, mode: WriteMode);
    fn set_content(bin_data: &[u8]);
}

// pub(crate) struct ErrorMessage(String);

pub(crate) trait TuiAsciiPanelTrait {
    fn set_content(bin_data: &[u8]);
}

pub(crate) trait TuiVersatilePanelTrait {}
