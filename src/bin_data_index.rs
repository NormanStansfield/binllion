use ratatui::layout::Position;

use crate::interfaces::{BinDataIndexTrait, ViewPosition};
use crate::{constants, interfaces::Index};

pub(crate) struct BinDataIndex {
    index: usize,
    size: usize,
}

impl BinDataIndexTrait for BinDataIndex {
    fn new() -> Self {
        Self { index: 0, size: 0 }
    }

    fn index(&self) -> Index {
        Index::new(self.index)
    }

    fn move_to_right(&mut self) -> Index {
        self.index = self.index.saturating_add(1);

        if self.index > self.size {
            self.index = self.size;
        }
        Index::new(self.index)
    }

    fn move_to_left(&mut self) -> Index {
        self.index = self.index.saturating_sub(1);
        Index::new(self.index)
    }

    fn move_to_up(&mut self) -> Index {
        self.index = self.index.saturating_sub(constants::LINE_LEN);
        Index::new(self.index)
    }

    fn move_to_down(&mut self) -> Index {
        self.index = self.index.saturating_add(constants::LINE_LEN);
        if self.index > self.size {
            self.index = self.size;
        }
        Index::new(self.index)
    }

    // fn reset_index(&mut self) {
    //     self.index = 0;
    // }

    fn set_size(&mut self, value: Index) {
        let value = value.into_inner();
        self.size = value;
    }

    fn get_max_line(&self) -> usize {
        let offset = self.size.rem_euclid(constants::LINE_LEN);
        let max_line = (self.size.saturating_sub(offset) / constants::LINE_LEN).saturating_sub(1);

        max_line
    }

    fn get_current_line(&self) -> usize {
        let offset = self.index.rem_euclid(constants::LINE_LEN);
        let current_line = self.index.saturating_sub(offset) / constants::LINE_LEN;
        current_line
    }

    fn get_position(&self) -> ViewPosition {
        let x = (self.index % constants::LINE_LEN) as u16;
        let y = (self.index / constants::LINE_LEN) as u16;

        ViewPosition(Position::new(x, y))
    }
}
