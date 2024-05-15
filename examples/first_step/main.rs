// 標準ライブラリの読み込み
use std::io::{self, Read};

// main 関数
fn main() {
    // STDINからbyte列を読み込み
    for input_byte in io::stdin().bytes() {
        // 読み込んだbyteをcharに変換して束縛
        let output_char = input_byte.unwrap() as char;

        // 読み込んだ値を出力
        println!("{}", output_char);
    }
}
