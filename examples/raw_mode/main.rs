// raw_mode

// 標準ライブラリの読み込み
use std::io::{self, Read};
// crosstermの読み込み
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

// main 関数
fn main() {
    // raw モードに移行
    enable_raw_mode().unwrap();

    // STDINからbyte列を読み込み
    for input_byte in io::stdin().bytes() {
        // 読み込んだbyteをcharに変換して束縛
        let output_char = input_byte.unwrap() as char;

        // 読み込んだ値を出力
        println!("{}", output_char);

        // qが入力されたらbreak
        if output_char == 'q' {
            // raw モードを解除
            disable_raw_mode().unwrap();
            break;
        }
    }
}
