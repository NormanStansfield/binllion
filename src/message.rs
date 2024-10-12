// 情報伝播向け構造体

use ratatui::layout::Position;
use std::collections::VecDeque;

// 状態管理
pub(crate) struct Message {
    bin_data: BinData,
    cursor: CursorPosition,
}

impl Message {
    pub(crate) fn new() -> Self {
        Self {
            bin_data: BinData::new(),
            cursor: CursorPosition::new(),
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
    position: Position,
    line_len: usize,
    scroll_y: u16,
}

impl CursorPosition {
    const STEP: usize = 3;
    const ORIGIN_X: u16 = 10;
    const ORIGIN_Y: u16 = 2;
    const SCROLL_Y_BORDER: u16 = 6;

    pub(crate) fn new() -> Self {
        let index = 0;
        let position = Position {
            x: Self::ORIGIN_X,
            y: Self::ORIGIN_Y,
        };
        let line_len = 16;
        let scroll_y = 0;

        Self {
            index,
            position,
            line_len,
            scroll_y,
        }
    }

    pub(crate) fn position(&self) -> &Position {
        &self.position
    }

    pub(crate) fn scroll_y(&self) -> &u16 {
        &self.scroll_y
    }

    pub(crate) fn move_to_right(&mut self, len: usize) {
        self.index = self.index.saturating_add(1);

        if self.index > len {
            self.index = len;
        }
        self.calc_position();
    }

    pub(crate) fn move_to_left(&mut self) {
        self.index = self.index.saturating_sub(1);
        self.calc_position();
    }

    pub(crate) fn move_to_up(&mut self) {
        self.index = self.index.saturating_sub(self.line_len);
        self.calc_position();
    }

    pub(crate) fn move_to_down(&mut self, len: usize) {
        self.index = self.index.saturating_add(self.line_len);
        if self.index > len {
            self.index = len;
        }
        self.calc_position();
    }
    // カーソル位置計算
    fn calc_position(&mut self) {
        self.position.x = Self::ORIGIN_X + (Self::STEP * (self.index % self.line_len)) as u16;
        self.position.y = Self::ORIGIN_Y + (self.index / self.line_len) as u16;
    }
    // スクロール計算
    pub(crate) fn calc_scroll(&mut self, bottom: u16) {
        let border = bottom - Self::SCROLL_Y_BORDER;

        let scroll_y;

        if self.position.y > border {
            scroll_y = self.position.y - border;
            self.position.y = border;
        } else {
            scroll_y = 0;
        }
        self.scroll_y = scroll_y;
    }
}
