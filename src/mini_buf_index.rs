use crate::interfaces::{BinDataIndexTrait, Index};

const LINE_LEN: usize = 16;

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
        self.index = self.index.saturating_sub(LINE_LEN);
        Index::new(self.index)
    }

    fn move_to_down(&mut self) -> Index {
        self.index = self.index.saturating_add(LINE_LEN);
        Index::new(self.index)
    }

    fn reset_index(&mut self) {
        self.index = 0;
    }

    fn set_size(&mut self, value: Index) {
        let value = value.into_inner();
        self.size = value;
    }
}
