// keypresses-crossterm

// 標準ライブラリの読み込み
// use std::io::{self, Read};
use std::io::stdout;

// crosstermの読み込み
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

// main 関数
fn main() {
    // AlternateScreenへ移行
    crossterm::execute!(stdout(), EnterAlternateScreen).unwrap();
    // raw モードに移行
    enable_raw_mode().unwrap();

    loop {
        // イベント処理
        handle_events();
    }
}

// イベントハンドラ
fn handle_events() {
    // match式でResultを処理
    match event::read() {
        // キー入力処理
        Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
            handle_key_events(key_event);
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
fn handle_key_events(key_event: KeyEvent) {
    // Ctrl や SHIFT等のコンビネーションキー処理
    match key_event.modifiers {
        // Ctrlが押されている場合
        KeyModifiers::CONTROL => {
            // 対のキーの処理
            match key_event.code {
                // Ctrl + qが入力されたら
                KeyCode::Char('q') => {
                    // raw モードを解除
                    disable_raw_mode().unwrap();
                    // AlternateScreenから復帰
                    crossterm::execute!(stdout(), LeaveAlternateScreen).unwrap();
                    // アプリケーション終了処理
                    todo!("Process Exit");
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

// 入力されたキーを画面出力
fn dbg_print_key_code(key_event: KeyEvent) {
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
