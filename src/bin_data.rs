use crate::interfaces::{BinDataTrait, FilePath, HexData, Index, Notice};

use std::{
    collections::VecDeque,
    ffi::OsString,
    io::{Read, Write},
    ops::Range,
};

// 編集用構造体
pub(crate) struct BinData {
    buf: VecDeque<u8>,
    path: OsString,
}

impl BinDataTrait for BinData {
    fn new() -> Self {
        Self {
            buf: VecDeque::<u8>::new(),
            path: OsString::new(),
        }
    }

    // データ追加
    fn add_data(&mut self, value: HexData) {
        // let mut new_data: std::collections::VecDeque<u8> = std::collections::VecDeque::from(value);
        // let mut new_data: std::collections::VecDeque<HexData> = value.into();
        // let HexData(value) = value;
        let value = value.into_inner();

        self.buf.make_contiguous();
        self.buf.push_back(value);
        // self.buf.append(&mut new_data);
    }

    // データ挿入
    fn insert_data(&mut self, index: Index, value: HexData) {
        let index = index.into_inner();
        // let HexData(value) = value;
        let value = value.into_inner();

        self.buf.make_contiguous();
        self.buf.insert(index, value);
        self.buf.make_contiguous();
    }

    // データ削除
    fn delete_data(&mut self, index: Index) {
        let index = index.into_inner();
        // let Index(index) = index;

        self.buf.make_contiguous();
        if self.buf.len() > 1 {
            self.buf.remove(index);
            // self.buf.make_contiguous();
        }
    }

    // データ上書き
    fn update_data(&mut self, index: Index, value: HexData) {
        let index = index.into_inner();
        // let Index(index) = index;
        // let HexData(value) = value;
        let value = value.into_inner();

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
    fn import_from(&mut self, path: FilePath) -> std::io::Result<()> {
        let file_path = path.into_inner();
        if let Some(os_path) = file_path {
            // ファイルパスがある場合
            let mut file = std::fs::File::open(os_path.clone())?;
            let mut tmp_buf = Vec::<u8>::new();
            let _ = file.read_to_end(&mut tmp_buf)?;
            let mut new_data: VecDeque<u8> = VecDeque::from(tmp_buf);
            self.buf.clear();
            self.buf.append(&mut new_data);

            self.path = os_path;

            Ok(())
        } else {
            // ファイルパスがない場合
            Ok(())
        }
    }

    // ファイルへ書き込み
    fn export_to(&self) -> std::io::Result<()> {
        let mut file = std::fs::File::create(&self.path)?;

        let (res, _) = self.buf.as_slices();

        file.write_all(res)?;
        Ok(())
    }

    fn get_data(&self, index: Index) -> Result<HexData, ()> {
        let index = index.into_inner();
        let res = self.buf.get(index);
        if let Some(value) = res {
            Ok(HexData::new(*value))
        } else {
            Err(())
        }
    }

    fn get_file_name(&self) -> Notice {
        let file_name = std::path::Path::new(&self.path)
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            // 取得できない場合はフルパスを使う
            .unwrap_or_else(|| self.path.to_string_lossy().to_string());

        if file_name.is_empty() {
            Notice::new(String::from("no file"))
        } else {
            Notice::new(file_name)
        }
    }

    fn get_size(&mut self) -> Index {
        self.buf.make_contiguous();
        Index::new(self.buf.len())
    }

    fn get_slice(&mut self, range: Range<usize>) -> Vec<u8> {
        let start = range.start;

        self.buf.make_contiguous();
        let (buf, _) = self.buf.as_slices();

        let res = if let Some(value) = buf.get(range) {
            value
        } else {
            buf.get(start..).unwrap()
        };

        res.to_vec()
    }

    fn set_path(&mut self, path: FilePath) {
        let file_path = path.into_inner();
        if let Some(os_path) = file_path {
            self.path = os_path;
        }
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
