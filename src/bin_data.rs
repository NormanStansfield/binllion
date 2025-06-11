use crate::interfaces::{BinDataTrait, FilePath, HexData, Index};
use std::{
    f32::consts::E,
    io::{Read, Write},
};

// 編集用構造体
pub(crate) struct BinData {
    buf: std::collections::VecDeque<u8>,
}

impl BinDataTrait for BinData {
    fn new() -> Self {
        Self {
            buf: std::collections::VecDeque::<u8>::new(),
        }
    }

    // データ追加
    fn add_data(&mut self, value: HexData) {
        // let mut new_data: std::collections::VecDeque<u8> = std::collections::VecDeque::from(value);
        // let mut new_data: std::collections::VecDeque<HexData> = value.into();
        let HexData(value) = value;

        self.buf.make_contiguous();
        self.buf.push_back(value);
        // self.buf.append(&mut new_data);
    }

    // データ挿入
    fn insert_data(&mut self, index: Index, value: HexData) {
        let Index(index) = index;
        let HexData(value) = value;

        // self.buf.make_contiguous();
        self.buf.insert(index, value);
        self.buf.make_contiguous();
    }

    // データ削除
    fn delete_data(&mut self, index: Index) {
        let Index(index) = index;

        self.buf.make_contiguous();
        if self.buf.len() > 1 {
            self.buf.remove(index);
            // self.buf.make_contiguous();
        }
    }

    // データ上書き
    fn update_data(&mut self, index: Index, value: HexData) {
        let Index(index) = index;
        let HexData(value) = value;

        self.buf.make_contiguous();
        if let Some(elem) = self.buf.get_mut(index) {
            *elem = value;
        }
    }

    // 編集データを[u8]配列で返す
    // pub(crate) fn buf(&self) -> &[u8] {
    //     let (res, _) = self.buf.as_slices();
    //     res
    // }

    // ファイルから読み込み
    fn import_from(&mut self, path: FilePath) -> Result<(), std::io::Error> {
        if let FilePath(Some(path)) = path {
            // ファイルパスがある場合
            let mut file = std::fs::File::open(path)?;
            let mut tmp_buf = Vec::<u8>::new();
            let _ = file.read_to_end(&mut tmp_buf)?;
            let mut new_data: std::collections::VecDeque<u8> =
                std::collections::VecDeque::from(tmp_buf);
            self.buf.clear();
            self.buf.append(&mut new_data);

            Ok(())
        } else {
            // ファイルパスがない場合
            Ok(())
        }
    }

    // ファイルへ書き込み
    fn export_to(&self, path: FilePath) -> Result<(), std::io::Error> {
        if let FilePath(Some(path)) = path {
            // ファイルパスがある場合
            let mut file = std::fs::File::create(path)?;

            let (res, _) = self.buf.as_slices();

            file.write_all(res)?;
            Ok(())
        } else {
            // ファイルパスがない場合
            Ok(())
        }
    }

    fn data_from_index(&mut self, index: Index) -> Result<HexData, ()> {
        unimplemented!();
    }
}

// Vec<u8>から編集データへ変換
// impl From<Vec<u8>> for BinData {
//     fn from(buf: Vec<u8>) -> Self {
//         BinData {
//             buf: std::collections::VecDeque::from(buf),
//         }
//     }
// }
