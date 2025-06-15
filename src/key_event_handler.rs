use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::interfaces::{CharCode, Command, KeyEventHandlerTrait};

pub(crate) struct KeyEventHandler;

impl KeyEventHandlerTrait for KeyEventHandler {
    fn handle_key_event() -> Command {
        // match式でResultを処理
        match event::read() {
            // キー入力処理
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                // Self::key_events(&key_event)

                // Ctrl や SHIFT等のコンビネーションキー処理
                match key_event.modifiers {
                    // Ctrlが押されている場合
                    KeyModifiers::CONTROL => {
                        // 対のキーの処理
                        match key_event.code {
                            // Ctrl + qが入力されたら
                            KeyCode::Char(char_code)
                                if matches!(char_code.to_ascii_lowercase(), 'q') =>
                            {
                                // KeyCode::Char('q') | KeyCode::Char('Q') => {
                                // イベントループ終了
                                // self.looping = false;
                                return Command::Exit;
                            }
                            _ => {
                                // return Command::Nope
                            }
                        }
                    }
                    // KeyModifiers::SHIFT 等
                    _ => {
                        // todo!()
                        //  return Command::Nope
                    }
                }

                // 通常のキー入力処理
                match key_event.code {
                    // match code.into_inner() {
                    // 文字関連
                    // カーソル左移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'h') => {
                        // KeyCode::Char('h') | KeyCode::Char('H') => {
                        // cursor.move_to_left();
                        // self.reset_input_buf(message);
                        Command::MoveToLeft
                    }
                    // カーソル右移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'l') => {
                        // KeyCode::Char('l') | KeyCode::Char('L') => {
                        // cursor.move_to_right(len);
                        // self.reset_input_buf(message);
                        Command::MoveToRight
                    }
                    // カーソル下移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'j') => {
                        // KeyCode::Char('j') | KeyCode::Char('J') => {
                        // cursor.move_to_down(len);
                        // self.reset_input_buf(message);
                        Command::MoveToDown
                    }
                    // カーソル上移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'k') => {
                        // KeyCode::Char('k') | KeyCode::Char('K') => {
                        // cursor.move_to_up();
                        // self.reset_input_buf(message);
                        Command::MoveToUp
                    }

                    // 削除
                    KeyCode::Delete => {
                        // KeyCode::Delete | KeyCode::Char('x') | KeyCode::Char('X') => {
                        Command::DeleteData
                    }

                    // 削除
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'x') => {
                        // KeyCode::Delete | KeyCode::Char('x') | KeyCode::Char('X') => {
                        Command::DeleteData
                    }

                    // 書き込みモード変更
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'i') => {
                        // KeyCode::Char('i') | KeyCode::Char('I') => {
                        // message.toggle_mode();
                        Command::ChangeWriteMode
                    }

                    // ファイルへ保存
                    // KeyCode::Char('w') | KeyCode::Char('W') => Command::ExportFile,
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'w') => {
                        Command::ExportFile
                    }

                    // 数値データ入力
                    // KeyCode::Char(char_code @ ('0'..='9' | 'a'..='f' | 'A'..='F')) => {
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), '0'..='9' | 'a'..='f') => {
                        Command::InputData(CharCode::new(char_code))
                    }

                    // 矢印キー等制御文字は対象外
                    _ => {
                        // todo!()
                        Command::Nope
                    }
                }
            }
            // エラーの場合
            Err(err) => {
                println!("Error: {}", err);
                Command::Nope
            }
            // その他入力（マウス等）
            _ => {
                // todo!()
                Command::Nope
            }
        }
    }
}

// impl KeyEventHandler {
//     // キー入力処理
//     fn key_events(key_event: &KeyEvent) -> Command {
//         // let len = message.bin_data().buf().len();
//         // let cursor = message.cursor_mut();

//         // Ctrl や SHIFT等のコンビネーションキー処理
//         match key_event.modifiers {
//             // Ctrlが押されている場合
//             KeyModifiers::CONTROL => {
//                 // 対のキーの処理
//                 match key_event.code {
//                     // Ctrl + qが入力されたら
//                     KeyCode::Char('q') | KeyCode::Char('Q') => {
//                         // イベントループ終了
//                         // self.looping = false;
//                         return Command::Exit;
//                     }
//                     _ => {
//                         // return Command::Nope
//                     }
//                 }
//             }
//             // KeyModifiers::SHIFT 等
//             _ => {
//                 // todo!()
//                 //  return Command::Nope
//             }
//         }

//         // 通常のキー入力処理
//         match key_event.code {
//             // 文字関連
//             // カーソル左移動
//             KeyCode::Char('h') | KeyCode::Char('H') => {
//                 // cursor.move_to_left();
//                 // self.reset_input_buf(message);
//                 Command::MoveToLeft
//             }
//             // カーソル右移動
//             KeyCode::Char('l') | KeyCode::Char('L') => {
//                 // cursor.move_to_right(len);
//                 // self.reset_input_buf(message);
//                 Command::MoveToRight
//             }
//             // カーソル下移動
//             KeyCode::Char('j') | KeyCode::Char('J') => {
//                 // cursor.move_to_down(len);
//                 // self.reset_input_buf(message);
//                 Command::MoveToDown
//             }
//             // カーソル上移動
//             KeyCode::Char('k') | KeyCode::Char('K') => {
//                 // cursor.move_to_up();
//                 // self.reset_input_buf(message);
//                 Command::MoveToUp
//             }

//             // 削除
//             KeyCode::Delete | KeyCode::Char('x') | KeyCode::Char('X') => {
//                 // let index = cursor.index();

//                 // // 最後尾の場合は、カーソルを移動
//                 // if index == len.saturating_sub(1) {
//                 //     cursor.move_to_left();
//                 // }

//                 // message.bin_data_mut().remove(index);

//                 // // データが1つの場合はゼロフィル
//                 // if len == 1 {
//                 //     message.bin_data_mut().update(0, 0);
//                 // }

//                 // self.reset_input_buf(message);
//                 Command::DeleteData
//             }

//             // 書き込みモード変更
//             KeyCode::Char('i') | KeyCode::Char('I') => {
//                 // message.toggle_mode();
//                 Command::ChangeWriteMode
//             }

//             // ファイルへ保存
//             KeyCode::Char('w') | KeyCode::Char('W') => {
//                 // if let Some(path) = message.current_file().path() {
//                 //     if let Err(e) = message.bin_data().export_to(path) {
//                 //         message.notice_mut().add(e.to_string());
//                 //     } else {
//                 //         let success_msg = String::from("Saved!");
//                 //         message.notice_mut().add(success_msg);
//                 //     }
//                 // } else {
//                 //     let err_msg = String::from("Not specified file path");
//                 //     message.notice_mut().add(err_msg);
//                 // }
//                 Command::ExportFile
//             }

//             // 数値データ入力
//             KeyCode::Char(char_code @ ('0'..='9' | 'a'..='f' | 'A'..='F')) => {
//                 // // 入力データをミニバッファへ書き込み
//                 // self.input_buf.add(char_code);

//                 // // 16進数へ変換
//                 // let res = self.input_buf.to_hex();

//                 // // 16進数へ変換が成功なら
//                 // if let Ok(val) = res {
//                 //     use crate::message::WriteMode::*;

//                 //     let index = message.cursor().index();
//                 //     match message.write_mode() {
//                 //         // 上書き処理
//                 //         OverWrite => {
//                 //             message.bin_data_mut().update(index, val);
//                 //         }
//                 //         // 挿入処理
//                 //         Insert => {
//                 //             // 最初の桁に入力あり
//                 //             if self.input_buf.index() != 0 {
//                 //                 // 下の桁を0にする
//                 //                 self.input_buf.set_value(0);
//                 //                 self.input_buf.add(char_code);

//                 //                 // 16進数へ変換
//                 //                 let res = self.input_buf.to_hex();
//                 //                 if let Ok(val) = res {
//                 //                     message.bin_data_mut().insert(index, val);
//                 //                 }
//                 //             }

//                 //             // 最後の桁に入力あり
//                 //             if self.input_buf.index() == 0 {
//                 //                 message.bin_data_mut().update(index, val);
//                 //             }
//                 //         }
//                 //     }
//                 // }
//                 // // ミニバッファの入力分、カーソルを移動
//                 // message.cursor_mut().input_buf_x(self.input_buf.index());
//                 Command::InputData(CharCode::new(char_code))
//             }

//             // 矢印キー等制御文字は対象外
//             _ => {
//                 // todo!()
//                 Command::Nope
//             }
//         }
//     }
// }
