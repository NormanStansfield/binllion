use crate::{
    bin_data::{self, BinData},
    interfaces::{AppTrait, BinDataTrait, FilePath, Notice, NoticeProviderTrait},
    notice_provider::{self, NoticeProvider},
};

pub(crate) struct App {
    // 描画用
    terminal: ratatui::DefaultTerminal,
    // ループが継続中フラグ
    running: bool,
    // file_path: Filefile_Path,
}

impl Drop for App {
    fn drop(&mut self) {
        // 画面復旧
        ratatui::restore();
    }
}

// clap v4
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// file_Path to file
    file: Option<std::ffi::OsString>,
}

impl AppTrait for App {
    fn new() -> Self {
        // ヘルプ表示用
        let _ = Args::parse();
        // 画面初期化
        let terminal = ratatui::init();
        let running = true;

        Self { terminal, running }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn run(&mut self) {
        // 引数からファイルの読み込み
        let args = Args::parse();
        let file_path: FilePath = if args.file.is_some() {
            FilePath(args.file)
        } else {
            FilePath(None)
        };

        // 編集データ格納用
        let mut bin_data = BinData::new();
        // ratatui::restore();

        // ファイルのインポート
        let res = bin_data.import_from(file_path);

        let mut notice_provider = NoticeProvider::new();

        if let Err(err) = res {
            // エラーであれば通知する
            let message = err.to_string();
            notice_provider.add(Notice::new(message));
        };

        self.quit();
        ratatui::restore();

        unimplemented!();
    }
}
