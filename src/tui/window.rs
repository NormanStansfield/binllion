use std::ops::Range;

use crate::constants;
use crate::interfaces::{MiniBufPosition, TuiArea};
use crate::interfaces::{ViewPosition, WindowTrait};
use ratatui::layout::Position;

pub(crate) struct Window {
    window_y: usize,
}

impl WindowTrait for Window {
    fn get_range(&self, area: TuiArea) -> Range<usize> {
        let area = area.into_inner();

        let upper_line = self.window_y;
        let bottom_line = self.window_y + (area.height - area.y) as usize;

        let begin_line: usize = upper_line * constants::LINE_LEN;
        let end_line: usize = bottom_line * constants::LINE_LEN + constants::LINE_LEN;

        begin_line..end_line
    }

    fn new() -> Self {
        Self { window_y: 0 }
    }

    fn move_to_up(&mut self, current_line: usize) {
        if current_line < self.window_y {
            self.window_y = current_line;
        }
    }

    fn move_to_down(&mut self, current_line: usize, area: TuiArea, max_line: usize) {
        let area = area.into_inner();
        let diff = (area.height - area.y) as usize;
        if current_line >= self.window_y + diff {
            self.window_y = current_line - diff;
        }
        if self.window_y > max_line {
            self.window_y = max_line;
        }
    }
}

impl Window {
    pub(crate) fn get_window_y(&self) -> usize {
        self.window_y
    }
}
