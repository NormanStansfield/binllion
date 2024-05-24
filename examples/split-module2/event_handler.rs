// split-module2

// イベントハンドラ
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
