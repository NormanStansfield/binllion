use std::collections::VecDeque;

// 状態管理
pub(crate) struct Message {
    bin_data: BinData,
}

impl Message {
    pub(crate) fn new() -> Self {
        Self { bin_data: BinData::new() }
    }

    pub(crate) fn bin_data(&self) -> &BinData{
        &self.bin_data
    }

    pub(crate) fn bin_data_mut(&mut self) -> &mut BinData{
        &mut self.bin_data
    }
}

// 編集用構造体
pub(crate) struct BinData {
    buf: VecDeque<u8>,
}

impl BinData {
    pub(crate) fn new() -> Self {
        BinData { buf: VecDeque::new() }
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
        BinData { buf: VecDeque::from(buf) }
    }
}

impl Default for BinData {
    fn default() -> Self {
        Self::new()
    }
}

