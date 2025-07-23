use crate::write_mode::WriteMode;
use nutype::nutype;
use ratatui::layout::Position;
use std::ops::Range;

pub(crate) trait AppTrait {
    fn run(&mut self);
    fn new() -> Self;
    fn is_running(&self) -> bool;
    fn quit(&mut self);
}

pub(crate) trait NoticeProviderTrait {
    fn new() -> Self;
    fn push(&mut self, notice: Notice);
    fn pop(&mut self) -> Notice;
}

#[cfg_attr(not(test), nutype(sanitize(trim), derive(Default), default = ""))]
#[cfg_attr(
    test,
    nutype(sanitize(trim), derive(Default, Debug, PartialEq), default = "")
)]
pub(crate) struct Notice(String);

#[nutype(derive(Clone))]
pub(crate) struct FilePath(Option<std::ffi::OsString>);

pub(crate) trait CurrentFileTrait {
    #[allow(dead_code)]
    fn path(&self) -> FilePath; // unimplemented!
    fn set_path(&mut self, path: FilePath);
}

pub(crate) trait BinDataTrait {
    fn new() -> Self;
    // fn add_data(&mut self, value: HexData);
    fn insert_data(&mut self, index: Index, value: HexData);
    fn delete_data(&mut self, index: Index);
    fn update_data(&mut self, index: Index, value: HexData);
    fn import_from(&mut self, path: FilePath) -> std::io::Result<()>;
    fn export_to(&self) -> std::io::Result<()>;
    fn to_hex_data(&self, index: Index) -> Option<HexData>;
    fn file_name(&self) -> Notice;
    fn len(&mut self) -> Index;
    fn to_vec(&mut self, range: Range<usize>) -> Vec<u8>;
}

// #[cfg_attr(not(test), nutype(derive(Clone, AsRef)))]
#[cfg_attr(not(test), nutype())]
#[cfg_attr(test, nutype(derive(Clone, AsRef, PartialEq, Debug)))]
pub(crate) struct HexData(u8);

// #[cfg_attr(not(test), nutype(derive(Clone, AsRef)))]
#[cfg_attr(not(test), nutype())]
#[cfg_attr(test, nutype(derive(Clone, AsRef, PartialEq, Debug)))]
pub(crate) struct Index(usize);

pub(crate) trait KeyEventHandlerTrait {
    fn handle_key_event(event: Result<crossterm::event::Event, std::io::Error>) -> Command;
}

// #[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) enum Command {
    MoveToUp,
    MoveToDown,
    MoveToLeft,
    MoveToRight,
    ChangeWriteMode,
    // ImportFile,
    ExportFile,
    Exit,
    DeleteData,
    Nop,
    InputData(CharCode),
}

pub(crate) trait MiniBufTrait {
    fn new() -> Self;
    fn add(&mut self, char_code: CharCode) -> MiniBufPosition;
    fn updata(&mut self, value: HexData);
    fn to_hex_data(&self) -> Result<HexData, std::num::ParseIntError>;
    fn mini_buf_position(&self) -> MiniBufPosition;
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub(crate) enum MiniBufPosition {
    Right,
    Left,
}

// #[cfg_attr(not(test), nutype(sanitize(with = |char| char.to_ascii_lowercase()), derive(AsRef, Debug)))]
#[cfg_attr(not(test), nutype(sanitize(with = |char| char.to_ascii_lowercase())))]
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
    fn position(&self) -> ViewPosition;
    fn max_line(&self) -> usize;
    fn current_line(&self) -> usize;
}

pub(crate) trait WindowTrait {
    fn window_range(&self, area: TuiArea) -> Range<usize>;
    fn new() -> Self;
    fn move_to_up(&mut self, current_line: usize);
    fn move_to_down(&mut self, current_line: usize, area: TuiArea, max_line: usize);
}

#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct ViewPosition(pub(crate) Position);

pub(crate) trait WriteModeTrait {
    fn toggle_mode(&self) -> WriteMode;
}

pub(crate) trait TuiLayoutProviderTrait {
    fn layout(terminal: &mut ratatui::DefaultTerminal) -> TuiLayout;
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

// #[nutype(derive(Clone, AsRef))]
#[nutype(derive(Clone))]
pub(crate) struct TuiArea(ratatui::layout::Rect);

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
    fn view_position(
        current_line: usize,
        area: TuiArea,
        position: ViewPosition,
        window_y: usize,
    ) -> ViewPosition;
}

pub(crate) trait TuiAsciiContentTrait {
    fn set_content(&mut self, bin_data: Vec<u8>, address: usize);
}

// pub(crate) trait TuiVersatilePanelTrait {}
