// モジュールファイルの読み込み
mod event_handler;
mod message;
mod tui;

// イベントハンドラ
use crate::event_handler::EventHandler;
// TUI関連
use crate::tui::{end_tui, init_tui, render_main};
// 状態管理
use crate::message::Message;

// main 関数
fn main() {
    // 状態管理
    let mut message = Message::new();

    // 仮データ
    let bin_data = message.bin_data_mut();
    bin_data.push_back(vec![
        0x01, 0x02, 0x03, 0x00, 0x63, 0x71, 0x00, 0x61, 0x62, 0x0f, 0x01, 0x02, 0x03, 0x00, 0x63,
        0x71, 0x0f, 0x61, 0x62, 0x63, 0x01, 0xff, 0x03, 0x00, 0x63, 0x71, 0x0f, 0x61, 0x62, 0x63,
    ]);

    // 画面初期化
    let _ = init_tui();

    // イベントハンドラ
    let mut event_handler: EventHandler = EventHandler::new();

    // イベントループ
    while event_handler.is_looping() {
        // 描画処理
        let _ = render_main(&message);
        // イベント処理
        event_handler.run(&mut message);
    }

    // 画面復旧
    let _ = end_tui();
}
