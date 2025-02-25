// イベントハンドラ

// テスト用出力関数
// use self::test::dbg_print_key_code;
// crosstermクレート
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
// 状態管理
use crate::message::Message;

// 入力用ミニバッファ
#[derive(Debug)]
struct InputBuf {
    buf: [char; 2],
    index: usize,
}

use std::num::ParseIntError;
impl InputBuf {
    fn new() -> Self {
        // let mut buf : [char; 2] = Default::default();
        let buf: [char; 2] = ['0'; 2];
        let index = 0;
        Self { buf, index }
    }

    // バッファにキーボードからの入力を入れる
    fn add(&mut self, value: char) {
        self.buf[self.index] = value;
        self.index = (self.index + 1) % 2;
    }

    // バッファの内容を16進数へ変換
    fn to_hex(&self) -> Result<u8, ParseIntError> {
        let str: String = self.buf.iter().collect();
        let res = u8::from_str_radix(&str, 16);
        // if let Ok(str) = &res {
        //     dbg!(format!("{:X}", str));
        // }
        res
    }

    // バッファに値をセット
    fn set(&mut self, value: u8) {
        // 16進数へ変換
        let str = format!("{:02X}", value);

        self.index = 0;
        for x in str.chars() {
            self.add(x);
        }
        self.index = 0;
    }

    fn index(&self) -> usize {
        self.index
    }
}

pub(crate) struct EventHandler {
    looping: bool,
    input_buf: InputBuf,
}

impl EventHandler {
    // コンストラクタ
    pub(crate) fn new() -> Self {
        Self {
            looping: true,
            input_buf: InputBuf::new(),
        }
    }

    pub(crate) fn is_looping(&self) -> bool {
        self.looping
    }

    // イベントループ
    pub(crate) fn run(&mut self, message: &mut Message) {
        self.handle_events(message);
    }
    // イベントハンドラ
    fn handle_events(&mut self, message: &mut Message) {
        // match式でResultを処理
        match event::read() {
            // キー入力処理
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(&key_event, message);
            }
            // エラーの場合
            Err(err) => {
                println!("Error: {}", err);
            }
            // その他入力（マウス等）
            _ => {
                // todo!()
            }
        };
    }
    // キー入力処理
    fn handle_key_events(&mut self, key_event: &KeyEvent, message: &mut Message) {
        let len = message.bin_data().buf().len();
        let cursor = message.cursor_mut();

        // Ctrl や SHIFT等のコンビネーションキー処理
        match key_event.modifiers {
            // Ctrlが押されている場合
            KeyModifiers::CONTROL => {
                // 対のキーの処理
                match key_event.code {
                    // Ctrl + qが入力されたら
                    KeyCode::Char('q') => {
                        // イベントループ終了
                        self.looping = false;
                        // todo!("Process Exit");
                    }
                    _ => {}
                }
            }
            // KeyModifiers::SHIFT 等
            _ => {
                // todo!()
            }
        }

        // 通常のキー入力処理
        match key_event.code {
            // 文字関連
            // カーソル左移動
            KeyCode::Char('h') => {
                cursor.move_to_left();
                self.reset_input_buf(message);
            }
            // カーソル右移動
            KeyCode::Char('l') => {
                cursor.move_to_right(len);
                self.reset_input_buf(message);
            }
            // カーソル下移動
            KeyCode::Char('j') => {
                cursor.move_to_down(len);
                self.reset_input_buf(message);
            }
            // カーソル上移動
            KeyCode::Char('k') => {
                cursor.move_to_up();
                self.reset_input_buf(message);
            }

            // 削除
            KeyCode::Delete => {
                let index = cursor.index();

                // 最後尾の場合は、カーソルを移動
                if index == len.saturating_sub(1) {
                    cursor.move_to_left();
                }

                message.bin_data_mut().remove(index);

                // データが1つの場合はゼロフィル
                if len == 1 {
                    message.bin_data_mut().update(0, 0);
                }

                self.reset_input_buf(message);
            }

            // 書き込みモード変更
            KeyCode::Char('i') => {
                message.toggle_mode();
            }

            // 数値データ入力
            KeyCode::Char(char_code @ ('0'..='9' | 'a'..='f' | 'A'..='F')) => {
                // 入力データをミニバッファへ書き込み
                self.input_buf.add(char_code);

                // 16進数へ変換が成功なら
                let res = self.input_buf.to_hex();
                if let Ok(val) = res {
                    use crate::message::WriteMode::*;

                    let index = message.cursor().index();
                    match message.write_mode() {
                        OverWrite => {
                            message.bin_data_mut().update(index, val);
                        }
                        Insert => {
                            message.bin_data_mut().insert(index, val);
                        }
                    }
                    message.cursor_mut().input_buf_x(self.input_buf.index());
                }
            }

            // 矢印キー等制御文字は対象外
            _ => {
                // todo!()
            }
        }

        // 入力されたキーを画面出力
        // dbg_print_key_code(key_event);
    }

    // 入力バッファをカーソル位置の値でリセット
    fn reset_input_buf(&mut self, message: &mut Message) {
        let buf = message.bin_data().buf();
        let index = message.cursor().index();
        self.input_buf.set(buf[index]);
        message.cursor_mut().input_buf_x(self.input_buf.index());
    }
}

mod test {
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    // 画面出力（デバック系）
    // 入力されたキーを画面出力
    #[test]
    #[ignore]
    pub(super) fn dbg_print_key_code(key_event: &KeyEvent) {
        // Ctrlなどのキー入力がある場合
        if key_event.modifiers != KeyModifiers::NONE {
            println!("KeyModifiers: {:?} \r", key_event.modifiers);
        }
        // 文字関連
        if let KeyCode::Char(code) = key_event.code {
            let u8_code = code as u8;
            println!(
                "Hex: {0:#X} Binary: {0:08b} ASCII: {0:#03} Character: {0:#?}\r",
                u8_code
            );
        }
        // 制御文字等
        else {
            println!("{:?}\r", key_event);
        }
    }
}
