// ratatui

// モジュールファイルの読み込み
mod event_handler;
mod tui;

// イベントハンドラ
use crate::event_handler::EventHandler;
// TUI関連
use crate::tui::{end_tui, init_tui};

// main 関数
fn main() {
    // 画面初期化
    let _ = init_tui();

    // イベントハンドラ
    let mut event_handler: EventHandler = EventHandler::new();
    // イベント処理
    event_handler.run();

    // 画面復旧
    let _ = end_tui();
}
