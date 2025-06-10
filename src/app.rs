use crate::{interfaces::{AppTrait,FilePath}}
;
// use crate::FilePath;

pub(crate) struct App {
    // 描画用
    terminal: ratatui::DefaultTerminal,
    // ループが継続中フラグ
    running: bool,
    // path: FilePath,
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
    /// Path to file
    file: Option<std::ffi::OsString>,
}

impl AppTrait for App {
    fn new() -> Self {
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
        let path: FilePath;
        if args.file.is_some() {
            path = FilePath(args.file);
        } else {
            path = FilePath(None);
        }

        self.quit();

        unimplemented!();
    }
}
