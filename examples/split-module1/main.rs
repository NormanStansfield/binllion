// split-module1

// TUI関連
mod tui {
    // crosstermの読み込み
    use crossterm::terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    };
    // 標準ライブラリの読み込み
    use std::io::{self, stdout};

    // 画面初期化
    pub(crate) fn init_tui() -> io::Result<()> {
        // AlternateScreenへ移行
        crossterm::execute!(stdout(), EnterAlternateScreen)?;
        // raw モードに移行
        enable_raw_mode()?;

        Ok(())
    }
    // 画面復旧
    pub(crate) fn end_tui() -> io::Result<()> {
        // raw モードを解除
        disable_raw_mode()?;
        // AlternateScreenから復帰
        crossterm::execute!(stdout(), LeaveAlternateScreen)?;

        Ok(())
    }
}

// イベントハンドラ
mod event_handler {
    // テスト用出力関数の読み込み
    use self::test::dbg_print_key_code;
    // crosstermの読み込み
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    pub(crate) struct EventHandler {
        looping: bool,
    }

    impl EventHandler {
        // コンストラクタ
        pub(crate) fn new() -> Self {
            Self { looping: true }
        }
        // イベントループ
        pub(crate) fn run(&mut self) {
            while self.looping {
                self.handle_events();
            }
        }
        // イベントハンドラ
        fn handle_events(&mut self) {
            // match式でResultを処理
            match event::read() {
                // キー入力処理
                Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_events(&key_event);
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
        fn handle_key_events(&mut self, key_event: &KeyEvent) {
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
                KeyCode::Char(char_code) => {
                    // todo!()
                }
                // 矢印キー等制御文字は対象外
                _ => {
                    // todo!()
                }
            }

            // 入力されたキーを画面出力
            dbg_print_key_code(key_event);
        }
    }

    mod test {
        use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

        // 画面出力（デバック系）
        // 入力されたキーを画面出力
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
}

// イベントハンドラの読み込み
use crate::event_handler::EventHandler;
// use self::tui;

// main 関数
fn main() {
    // 画面初期化
    let _ = tui::init_tui();

    // イベントハンドラ
    let mut event_handler: EventHandler = EventHandler::new();
    // イベント処理
    event_handler.run();

    // 画面復旧
    let _ = tui::end_tui();
}
