// 情報伝播向け構造体

use ratatui::layout::Position;
use std::collections::VecDeque;
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
    layout: [Rc<[Rect]>; 4], // main_layout, sub_layout, inner_main, inner_sub
}

impl Message {
    pub(crate) fn new() -> Self {
        Self {
            bin_data: BinData::new(),
            cursor: CursorPosition::new(),
            scroll: Scroll::new(),
            write_mode: WriteMode::OverWrite,
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
        self.buf.make_contiguous();
        self.buf.insert(index, value);
        self.buf.make_contiguous();
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

    pub(crate) fn input_buf_x(&mut self, value: usize) {
        self.input_buf_x = value;
    }

    pub(crate) fn position(&self) -> &Position {
        &self.position
    }

    pub(crate) fn move_to_right(&mut self, len: usize) {
        self.index = self.index.saturating_add(1);

        if self.index >= len {
            self.index = len - 1;
        }
        // self.calc_position();
    }

    pub(crate) fn move_to_left(&mut self) {
        self.index = self.index.saturating_sub(1);
        // self.calc_position();
    }

    pub(crate) fn move_to_up(&mut self) {
        self.index = self.index.saturating_sub(constants::LINE_LEN);
        // self.calc_position();
    }

    pub(crate) fn move_to_down(&mut self, len: usize) {
        self.index = self.index.saturating_add(constants::LINE_LEN);
        if self.index >= len {
            self.index = len - 1;
        }
        // self.calc_position();
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
        let border = bottom.saturating_sub(SCROLL_Y_BORDER);
        border
    }
    // スクロール計算
    pub(crate) fn calc_scroll(y: u16, border: u16) -> u16 {
        let scroll_y;
        if y > border {
            scroll_y = y - border;
        } else {
            scroll_y = 0;
        }
        scroll_y
    }
}

// 書き込みモード
pub(crate) enum WriteMode {
    OverWrite,
    Insert,
}
