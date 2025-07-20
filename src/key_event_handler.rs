use crate::interfaces::{CharCode, Command, KeyEventHandlerTrait};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub(crate) struct KeyEventHandler;

impl KeyEventHandlerTrait for KeyEventHandler {
    fn handle_key_event(event: Result<Event, std::io::Error>) -> Command {
        // match式でResultを処理
        match event {
            // キー入力処理
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
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
                                // イベントループ終了
                                return Command::Exit;
                            }
                            _ => {
                                // return Command::Nop
                            }
                        }
                    }
                    // KeyModifiers::SHIFT 等
                    _ => {
                        // todo!()
                        //  return Command::Nop
                    }
                }

                // 通常のキー入力処理
                match key_event.code {
                    // 文字関連
                    // カーソル左移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'h') => {
                        Command::MoveToLeft
                    }
                    // カーソル右移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'l') => {
                        Command::MoveToRight
                    }
                    // カーソル下移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'j') => {
                        Command::MoveToDown
                    }
                    // カーソル上移動
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'k') => {
                        Command::MoveToUp
                    }

                    // 削除
                    KeyCode::Delete => Command::DeleteData,

                    // 削除
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'x') => {
                        Command::DeleteData
                    }

                    // 書き込みモード変更
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'i') => {
                        Command::ChangeWriteMode
                    }

                    // ファイルへ保存
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), 'w') => {
                        Command::ExportFile
                    }

                    // 数値データ入力
                    KeyCode::Char(char_code) if matches!(char_code.to_ascii_lowercase(), '0'..='9' | 'a'..='f') => {
                        Command::InputData(CharCode::new(char_code))
                    }

                    // 矢印キー等制御文字は対象外
                    _ => Command::Nop,
                }
            }
            // エラーの場合
            Err(err) => {
                println!("Error: {}", err);
                Command::Nop
            }
            // その他入力（マウス等）
            _ => Command::Nop,
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyEvent;

    use super::*;

    #[test]
    fn test_handle_key_event() {
        // コントロール+qを入力
        let key_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::Exit);

        // hを入力
        let key_event = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::MoveToLeft);

        // lを入力
        let key_event = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::MoveToRight);

        // kを入力
        let key_event = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::MoveToUp);

        // jを入力
        let key_event = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::MoveToDown);

        // Deleteを入力
        let key_event = KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::DeleteData);

        // xを入力
        let key_event = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::DeleteData);

        // iを入力
        let key_event = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::ChangeWriteMode);

        // wを入力
        let key_event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::ExportFile);

        // 5を入力
        let key_event = KeyEvent::new(KeyCode::Char('5'), KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::InputData(CharCode::new('5')));

        // Nullを入力
        let key_event = KeyEvent::new(KeyCode::Null, KeyModifiers::NONE);
        let res = KeyEventHandler::handle_key_event(Ok(Event::Key(key_event)));
        assert_eq!(res, Command::Nop);
    }
}
