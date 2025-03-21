// イベントハンドラ

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
        u8::from_str_radix(&str, 16)
    }

    // バッファに値をセット
    fn set_value(&mut self, value: u8) {
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
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        // イベントループ終了
                        self.looping = false;
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
            KeyCode::Char('h') | KeyCode::Char('H') => {
                cursor.move_to_left();
                self.reset_input_buf(message);
            }
            // カーソル右移動
            KeyCode::Char('l') | KeyCode::Char('L') => {
                cursor.move_to_right(len);
                self.reset_input_buf(message);
            }
            // カーソル下移動
            KeyCode::Char('j') | KeyCode::Char('J') => {
                cursor.move_to_down(len);
                self.reset_input_buf(message);
            }
            // カーソル上移動
            KeyCode::Char('k') | KeyCode::Char('K') => {
                cursor.move_to_up();
                self.reset_input_buf(message);
            }

            // 削除
            KeyCode::Delete | KeyCode::Char('x') | KeyCode::Char('X') => {
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
            KeyCode::Char('i') | KeyCode::Char('I') => {
                message.toggle_mode();
            }

            // ファイルへ保存
            KeyCode::Char('w') | KeyCode::Char('W') => {
                if let Some(path) = message.current_file().path() {
                    if let Err(e) = message.bin_data().export_to(path) {
                        message.notice_mut().add(e.to_string());
                    } else {
                        let success_msg = String::from("Saved!");
                        message.notice_mut().add(success_msg);
                    }
                } else {
                    let err_msg = String::from("Not specified file path");
                    message.notice_mut().add(err_msg);
                }
            }

            // 数値データ入力
            KeyCode::Char(char_code @ ('0'..='9' | 'a'..='f' | 'A'..='F')) => {
                // 入力データをミニバッファへ書き込み
                self.input_buf.add(char_code);

                // 16進数へ変換
                let res = self.input_buf.to_hex();

                // 16進数へ変換が成功なら
                if let Ok(val) = res {
                    use crate::message::WriteMode::*;

                    let index = message.cursor().index();
                    match message.write_mode() {
                        // 上書き処理
                        OverWrite => {
                            message.bin_data_mut().update(index, val);
                        }
                        // 挿入処理
                        Insert => {
                            // 最初の桁に入力あり
                            if self.input_buf.index() != 0 {
                                // 下の桁を0にする
                                self.input_buf.set_value(0);
                                self.input_buf.add(char_code);

                                // 16進数へ変換
                                let res = self.input_buf.to_hex();
                                if let Ok(val) = res {
                                    message.bin_data_mut().insert(index, val);
                                }
                            }

                            // 最後の桁に入力あり
                            if self.input_buf.index() == 0 {
                                message.bin_data_mut().update(index, val);
                            }
                        }
                    }
                }
                // ミニバッファの入力分、カーソルを移動
                message.cursor_mut().input_buf_x(self.input_buf.index());
            }

            // 矢印キー等制御文字は対象外
            _ => {
                // todo!()
            }
        }
    }

    // 入力ミニバッファをカーソル位置の値でリセット
    fn reset_input_buf(&mut self, message: &mut Message) {
        let buf = message.bin_data().buf();
        let index = message.cursor().index();
        self.input_buf.set_value(buf[index.saturating_sub(1)]);
        message.cursor_mut().input_buf_x(self.input_buf.index());
    }
}
