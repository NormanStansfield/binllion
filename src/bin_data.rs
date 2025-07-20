use crate::interfaces::{BinDataTrait, CurrentFileTrait, FilePath, HexData, Index, Notice};
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
    // fn add_data(&mut self, value: HexData) {
    //     let value = value.into_inner();

    //     self.buf.make_contiguous();
    //     self.buf.push_back(value);
    // }

    // データ挿入
    fn insert_data(&mut self, index: Index, value: HexData) {
        let index = index.into_inner();
        let value = value.into_inner();

        self.buf.make_contiguous();
        self.buf.insert(index, value);
        self.buf.make_contiguous();
    }

    // データ削除
    fn delete_data(&mut self, index: Index) {
        let index = index.into_inner();

        self.buf.make_contiguous();
        self.buf.remove(index);
    }

    // データ上書き
    fn update_data(&mut self, index: Index, value: HexData) {
        let index = index.into_inner();
        let value = value.into_inner();

        self.buf.make_contiguous();
        if let Some(elem) = self.buf.get_mut(index) {
            *elem = value;
        }
    }

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
}

impl CurrentFileTrait for BinData {
    fn set_path(&mut self, path: FilePath) {
        let file_path = path.into_inner();
        if let Some(os_path) = file_path {
            self.path = os_path;
        }
    }

    fn get_path(&self) -> FilePath {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_bin_data() {
        let mut bin_data = BinData::new();

        assert!(bin_data.get_data(Index::new(0)).is_err());

        bin_data.update_data(Index::new(0), HexData::new(99));
        assert_eq!(bin_data.get_data(Index::new(0)), Err(()));

        bin_data.insert_data(Index::new(0), HexData::new(99));
        assert_eq!(bin_data.get_data(Index::new(0)), Ok(HexData::new(99)));

        bin_data.update_data(Index::new(0), HexData::new(55));
        assert_eq!(bin_data.get_data(Index::new(0)), Ok(HexData::new(55)));

        // bin_data.add_data(HexData::new(64)); // Add '@'
        // assert_eq!(bin_data.get_data(Index::new(1)), Ok(HexData::new(64)));
        bin_data.insert_data(Index::new(1), HexData::new(64));
        assert_eq!(bin_data.get_data(Index::new(1)), Ok(HexData::new(64)));

        assert_eq!(bin_data.get_size(), Index::new(2));

        assert_eq!(bin_data.get_slice(0..2), vec![55, 64]);

        bin_data.delete_data(Index::new(0));
        assert_eq!(bin_data.get_data(Index::new(0)), Ok(HexData::new(64)));

        assert_eq!(
            bin_data.get_file_name(),
            Notice::new(String::from("no file"))
        );

        assert!(bin_data.export_to().is_err());

        let file_name = "test_bin_data.test";
        let tmp_dir = tempdir();

        if let Ok(dir) = tmp_dir {
            let file_path = dir.path().join(file_name);
            // dbg!(&file_path);
            let file_path = FilePath::new(Some(file_path.into_os_string()));

            bin_data.set_path(file_path.clone());
            assert_eq!(
                bin_data.get_file_name(),
                Notice::new(String::from(file_name))
            );

            assert!(bin_data.export_to().is_ok());

            bin_data.delete_data(Index::new(0));
            assert!(bin_data.get_data(Index::new(0)).is_err());

            let res = bin_data.import_from(file_path);
            assert!(res.is_ok());
            assert_eq!(bin_data.get_data(Index::new(0)), Ok(HexData::new(64)));
        } else {
            panic!("Temporary file creation error on test_bin_data()");
        };
    }
}
