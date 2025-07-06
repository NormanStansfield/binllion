use std::ops::Range;

use crate::constants;
use crate::interfaces::{MiniBufPosition, TuiArea};
use crate::interfaces::{ViewPosition, WindowTrait};
use ratatui::layout::Position;

pub(crate) struct Window {
    window_y: usize,
}

impl WindowTrait for Window {
    fn with_mini_buf_position(
        position: ViewPosition,
        mini_buf_position: crate::interfaces::MiniBufPosition,
    ) -> ViewPosition {
        let position = if mini_buf_position == MiniBufPosition::Tail {
            let ViewPosition(Position { x, y }) = position;

            ViewPosition(Position::new(x - 1, y))
        } else {
            position
        };
        position
    }

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

    fn get_view_position(
        &self,
        current_line: usize,
        area: TuiArea,
        position: ViewPosition,
    ) -> ViewPosition {
        let area = area.into_inner();
        let ViewPosition(position) = position;

        let x = area.x + constants::ADDRESS_WIDTH as u16 + 2 + position.x * constants::STEP as u16;
        let y = area.y + (current_line.saturating_sub(self.window_y)) as u16;

        ViewPosition(Position::new(x, y))
    }
}
