// 情報伝播向け構造体

use ratatui::layout::Position;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{Read, Write};
// 定数
use crate::constants;
use ratatui::prelude::Rect;
use std::rc::Rc;

// 状態管理
pub(crate) struct Message {
    bin_data: BinData,
    cursor: CursorPosition,
    scroll: Scroll,
    write_mode: WriteMode,
    current_file: CurrentFile,
    notice: Notice,
    layout: [Rc<[Rect]>; 4], // main_layout, sub_layout, inner_main, inner_sub
}

impl Message {
    pub(crate) fn new() -> Self {
        Self {
            bin_data: BinData::new(),
            cursor: CursorPosition::new(),
            scroll: Scroll::new(),
            write_mode: WriteMode::OverWrite,
            current_file: CurrentFile::new(),
            notice: Notice::new(),
            layout: Default::default(),
        }
    }

    pub(crate) fn bin_data(&self) -> &BinData {
        &self.bin_data
    }

    pub(crate) fn bin_data_mut(&mut self) -> &mut BinData {
        &mut self.bin_data
    }

    pub(crate) fn cursor(&self) -> &CursorPosition {
        &self.cursor
    }

    pub(crate) fn cursor_mut(&mut self) -> &mut CursorPosition {
        &mut self.cursor
    }

    pub(crate) fn scroll(&self) -> &Scroll {
        &self.scroll
    }

    pub(crate) fn scroll_mut(&mut self) -> &mut Scroll {
        &mut self.scroll
    }

    pub(crate) fn write_mode(&self) -> &WriteMode {
        &self.write_mode
    }

    // 書き込みモード変更
    pub(crate) fn toggle_mode(&mut self) -> &WriteMode {
        use WriteMode::*;
        let mode = &self.write_mode;
        self.write_mode = match mode {
            OverWrite => Insert,
            Insert => OverWrite,
        };
        &self.write_mode
    }

    pub(crate) fn current_file(&self) -> &CurrentFile {
        &self.current_file
    }

    pub(crate) fn current_file_mut(&mut self) -> &mut CurrentFile {
        &mut self.current_file
    }

    pub(crate) fn notice(&self) -> &Notice {
        &self.notice
    }

    pub(crate) fn notice_mut(&mut self) -> &mut Notice {
        &mut self.notice
    }

    pub(crate) fn layout(&self) -> &[Rc<[Rect]>; 4] {
        &self.layout
    }

    pub(crate) fn layout_mut(&mut self) -> &mut [Rc<[Rect]>; 4] {
        &mut self.layout
    }
}

// 編集用構造体
pub(crate) struct BinData {
    buf: VecDeque<u8>,
}

impl BinData {
    pub(crate) fn new() -> Self {
        BinData {
            buf: VecDeque::new(),
        }
    }

    pub(crate) fn push_back(&mut self, new_buf: Vec<u8>) {
        let mut new_data: VecDeque<u8> = VecDeque::from(new_buf);
        self.buf.make_contiguous();
        self.buf.append(&mut new_data);
    }

    pub(crate) fn insert(&mut self, index: usize, value: u8) {
        // self.buf.make_contiguous();
        self.buf.insert(index, value);
        self.buf.make_contiguous();
    }

    pub(crate) fn remove(&mut self, index: usize) {
        self.buf.make_contiguous();
        if self.buf.len() > 1 {
            self.buf.remove(index);
            // self.buf.make_contiguous();
        }
    }

    pub(crate) fn update(&mut self, index: usize, value: u8) {
        self.buf.make_contiguous();
        if let Some(elem) = self.buf.get_mut(index) {
            *elem = value;
        }
    }

    pub(crate) fn buf(&self) -> &[u8] {
        let (res, _) = self.buf.as_slices();
        res
    }

    pub(crate) fn import_from(&mut self, path: &String) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        let mut tmp_buf = Vec::new();
        let _ = file.read_to_end(&mut tmp_buf)?;
        self.buf.clear();
        self.push_back(tmp_buf);

        Ok(())
    }

    pub(crate) fn export_to(&self, path: &String) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.buf())?;

        Ok(())
    }
}

impl From<Vec<u8>> for BinData {
    fn from(buf: Vec<u8>) -> Self {
        BinData {
            buf: VecDeque::from(buf),
        }
    }
}

impl Default for BinData {
    fn default() -> Self {
        Self::new()
    }
}

// カーソル位置管理
pub(crate) struct CursorPosition {
    index: usize,
    input_buf_x: usize,
    position: Position,
}

impl CursorPosition {
    const STEP: usize = 3;
    const ORIGIN_X: u16 = 10;
    const ORIGIN_Y: u16 = 2;

    pub(crate) fn new() -> Self {
        let index = 0;
        let input_buf_x = 0;
        let position = Position {
            x: Self::ORIGIN_X,
            y: Self::ORIGIN_Y,
        };

        Self {
            index,
            input_buf_x,
            position,
        }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn input_buf_x(&mut self, x: usize) {
        self.input_buf_x = x;
    }

    pub(crate) fn position(&self) -> &Position {
        &self.position
    }

    pub(crate) fn move_to_right(&mut self, len: usize) {
        self.index = self.index.saturating_add(1);

        if self.index > len {
            self.index = len;
        }
    }

    pub(crate) fn move_to_left(&mut self) {
        self.index = self.index.saturating_sub(1);
    }

    pub(crate) fn move_to_up(&mut self) {
        self.index = self.index.saturating_sub(constants::LINE_LEN);
    }

    pub(crate) fn move_to_down(&mut self, len: usize) {
        self.index = self.index.saturating_add(constants::LINE_LEN);
        if self.index > len {
            self.index = len;
        }
    }
    // カーソル位置計算
    pub(crate) fn calc_position(&mut self) {
        self.position.x = Self::ORIGIN_X
            + (Self::STEP * (self.index % constants::LINE_LEN) + self.input_buf_x) as u16;
        self.position.y = Self::ORIGIN_Y + (self.index / constants::LINE_LEN) as u16;
    }
    // カーソル上限計算
    pub(crate) fn adjust_y(&mut self, border: u16) {
        if self.position.y > border {
            self.position.y = border;
        }
    }
}

// スクロール量
pub(crate) struct Scroll {
    scroll_y: [u16; 2], // main:0, sub0:1
}

impl Scroll {
    fn new() -> Self {
        let scroll_y = [0; 2];
        Self { scroll_y }
    }

    pub(crate) fn scroll_y(&self) -> &[u16; 2] {
        &self.scroll_y
    }
    pub(crate) fn scroll_y_mut(&mut self) -> &mut [u16; 2] {
        &mut self.scroll_y
    }
    // スクロール上限計算
    pub(crate) fn calc_border(bottom: u16) -> u16 {
        const SCROLL_Y_BORDER: u16 = 3;
        bottom.saturating_sub(SCROLL_Y_BORDER)
    }
    // スクロール計算
    pub(crate) fn calc_scroll(y: u16, border: u16) -> u16 {
        if y > border {
            y - border
        } else {
            0
        }
    }
}

// 書き込みモード
pub(crate) enum WriteMode {
    OverWrite,
    Insert,
}

// 編集対象ファイル
pub(crate) struct CurrentFile {
    path: String,
}

// 編集対象ファイル管理
impl CurrentFile {
    fn new() -> Self {
        let path = String::new();
        Self { path }
    }

    pub(crate) fn path(&self) -> Option<&String> {
        if self.path.is_empty() {
            None
        } else {
            Some(&self.path)
        }
    }

    pub(crate) fn path_mut(&mut self) -> &mut String {
        &mut self.path
    }

    pub(crate) fn file_name(&self) -> String {
        let file_name = std::path::Path::new(&self.path)
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            // 取得できない場合はフルパスを使う
            .unwrap_or_else(|| self.path.to_string());

        if file_name.is_empty() {
            String::from("no file")
        } else {
            file_name
        }
    }
}

// ステータス伝達
pub(crate) struct Notice {
    count: Cell<u8>,
    notice: RefCell<VecDeque<String>>,
    cache: RefCell<String>,
}

impl Notice {
    pub(crate) fn new() -> Self {
        Self {
            count: Cell::new(0),
            notice: RefCell::new(VecDeque::new()),
            cache: RefCell::new(String::new()),
        }
    }

    pub(crate) fn add(&mut self, state: String) {
        self.notice.borrow_mut().push_back(state);
    }

    pub(crate) fn pop_front(&self) -> String {
        const LIMIT: u8 = 2;
        match self.count.get() {
            0 => {
                if let Some(state) = self.notice.borrow_mut().pop_front() {
                    *self.cache.borrow_mut() = format!(" {state} ");
                    self.count.set(1);
                } else {
                    self.cache.borrow_mut().clear();
                }
            }
            x if x > LIMIT => {
                self.count.set(0);
            }
            x => {
                self.count.set(x + 1);
            }
        }

        self.cache.borrow().clone()
    }
}
