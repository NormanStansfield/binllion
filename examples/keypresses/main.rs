// keypresses

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
        // シャドーイング
        let input_byte = input_byte.unwrap();
        // 読み込んだbyteをcharに変換して束縛
        let output_char = input_byte as char;

        // 読み込んだ値を出力
        if output_char.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", input_byte);
        } else {
            println!(
                "Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r",
                input_byte, output_char
            );
        }

        // qが入力されたらbreak
        if output_char == 'q' {
            // raw モードを解除
            disable_raw_mode().unwrap();
            break;
        }
    }
}
