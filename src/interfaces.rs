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

use std::ops::Range;

use crossterm;
use nutype::nutype;
use ratatui::prelude::Position;

use crate::write_mode::WriteMode;

#[cfg_attr(not(test), nutype(sanitize(trim), derive(Default), default = ""))]
#[cfg_attr(test, nutype(sanitize(trim), derive(Default, Debug, PartialEq), default = ""))]
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
    fn get_slice(&mut self, range: Range<usize>) -> Vec<u8>;
    fn set_path(&mut self, path: FilePath);
}

#[cfg_attr(not(test), nutype(derive(Clone, AsRef)))]
#[cfg_attr(test, nutype(derive(Clone, AsRef, PartialEq, Debug)))]
pub(crate) struct HexData(u8);

#[cfg_attr(not(test), nutype(derive(Clone, AsRef)))]
#[cfg_attr(test, nutype(derive(Clone, AsRef, PartialEq, Debug)))]
pub(crate) struct Index(usize);

pub(crate) trait KeyEventHandlerTrait {
    // fn new() -> Self;
    fn handle_key_event(event: Result<crossterm::event::Event, std::io::Error>) -> Command;
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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
#[cfg_attr(test, derive(Debug))]
pub(crate) enum MiniBufPosition {
    Head,
    Tail,
}

#[cfg_attr(not(test), nutype(sanitize(with = |char| char.to_ascii_lowercase()), derive(AsRef, Debug)))]
#[cfg_attr(test, nutype(sanitize(with = |char| char.to_ascii_lowercase()), derive(AsRef, Debug, PartialEq)))]
pub(crate) struct CharCode(char);

pub(crate) trait BinDataIndexTrait {
    fn new() -> Self;
    fn index(&self) -> Index;
    fn move_to_right(&mut self) -> Index;
    fn move_to_left(&mut self) -> Index;
    fn move_to_up(&mut self) -> Index;
    fn move_to_down(&mut self) -> Index;
    fn set_size(&mut self, value: Index);
    fn get_position(&self) -> ViewPosition;
    fn get_max_line(&self) -> usize;
    fn get_current_line(&self) -> usize;
}

pub(crate) trait WindowTrait {
    fn get_range(&self, area: TuiArea) -> Range<usize>;
    fn new() -> Self;
    fn move_to_up(&mut self, current_line: usize);
    fn move_to_down(&mut self, current_line: usize, area: TuiArea, max_line: usize);
}
pub(crate) struct ViewPosition(pub(crate) Position);

pub(crate) trait WriteModeTrait {
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

pub(crate) trait TuiMainPanelTrait {
    fn set_title(&mut self, title: Notice);
    fn set_err_msg(&mut self, message: Notice);
    fn set_mode(&mut self, mode: WriteMode);
}

pub(crate) trait TuiMainContentTrait {
    fn set_content(&mut self, bin_data: Vec<u8>, address: usize);
    fn with_mini_buf_position(
        position: ViewPosition,
        mini_buf_position: MiniBufPosition,
    ) -> ViewPosition;
    fn get_view_position(
        current_line: usize,
        area: TuiArea,
        position: ViewPosition,
        window_y: usize,
    ) -> ViewPosition;
}

// pub(crate) struct ErrorMessage(String);

pub(crate) trait TuiAsciiContentTrait {
    fn set_content(&mut self, bin_data: Vec<u8>, address: usize);
}

// pub(crate) trait TuiVersatilePanelTrait {}
