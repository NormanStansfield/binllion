// write_file_create

use std::io::Write;

// NOTE: 事前に下記コマンドでファイルを用意しておく
// echo "01234567890123456789" > ./test.data

// main 関数
#[rustfmt::skip]
fn main() -> Result<(), std::io::Error> {
    let path = "./test.data";
    let mut file = std::fs::File::create(path)?; // 上書き+余分なデータ削除
    // let mut file = std::fs::OpenOptions::new().write(true).open(path)?;  // 上書き+余分なデータは残る
    // let mut file = std::fs::OpenOptions::new().write(true).create(true).open(path)?;  // 上書き+余分なデータは残る
    // let mut file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(path)?;  // 上書き+余分なデータ削除
    // let mut file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path)?;  // 上書き+余分なデータ削除
    let _ = file.write_all(b"test write create")?;

    Ok(())
}
