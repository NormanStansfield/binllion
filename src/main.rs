// モジュールファイルの読み込み
mod constants;
mod event_handler;
mod message;
mod tui;

// イベントハンドラ
use crate::event_handler::EventHandler;
// TUI関連
use crate::tui::{render_main, render_prep};
// 状態管理
use crate::message::Message;
// clap v4
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file
    file: Option<String>,
}

// main 関数s
fn main() {
    // 状態管理
    let mut message = Message::new();

    // 引数からファイルの読み込み
    let args = Args::parse();
    match args.file {
        // 引数がある場合
        Some(path) => {
            let bin_data = message.bin_data_mut();
            if let Err(e) = bin_data.import_from(&path) {
                eprintln!("{e}");
                std::process::exit(-1)
            } else {
                let current_file = message.current_file_mut().path_mut();
                *current_file = path;
            }
        }
        // 引数がない場合
        None => {
            let bin_data = message.bin_data_mut();
            let data: Vec<u8> = vec![0x00];
            bin_data.push_back(data);
        }
    }

    // 画面初期化
    let mut terminal = ratatui::init();

    // イベントハンドラ
    let mut event_handler: EventHandler = EventHandler::new();

    // イベントループ
    while event_handler.is_looping() {
        // 描画処理準備
        let _ = render_prep(&mut terminal, &mut message);
        // 描画処理
        let _ = render_main(&mut terminal, &message);
        // イベント処理
        event_handler.run(&mut message);
    }

    // 画面復旧
    ratatui::restore();
}
