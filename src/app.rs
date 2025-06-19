use crate::interfaces::{
    BinDataIndexTrait, Command, HexData, KeyEventHandlerTrait, MiniBufTrait, WriteModeTrait,
};
use crate::key_event_handler::KeyEventHandler;
use crate::mini_buf::{self, MiniBuf};
use crate::mini_buf_index::BinDataIndex;
use crate::tui::TuiMainPanel;
use crate::write_mode::{self, WriteMode};
use crate::{
    bin_data::{self, BinData},
    interfaces::{
        AppTrait, BinDataTrait, FilePath, Notice, NoticeProviderTrait, TuiMainPanelTrait,
        TuiPanelCommonTrait,
    },
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
        // ヘルプ表示用 ratatui::init()の実行前に実施
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
        // let file_path: FilePath = if args.file.is_some() {
        let file_path: FilePath = if let Some(path) = args.file {
            FilePath::new(Some(path))
        } else {
            FilePath::new(None)
        };

        // 編集データ格納用
        let mut bin_data = BinData::new();

        // 編集データインデックス
        let mut bin_data_index = BinDataIndex::new();

        // 入力用ミニバッファ
        let mut mini_buf = MiniBuf::new();

        // 通知管理
        let mut notice_provider = NoticeProvider::new();

        // ファイルのインポート
        let res = bin_data.import_from(file_path.clone());
        if let Err(err) = res {
            // エラーであれば通知する
            let message = err.to_string();
            notice_provider.add(Notice::new(message));
        };

        // 書き込みモード
        let mut write_mode = WriteMode::Insert;

        while self.is_running() {
            bin_data_index.set_size(bin_data.get_size());

            let res = KeyEventHandler::handle_key_event();
            dbg!(&res);

            match res {
                Command::Exit => self.quit(),
                Command::ChangeWriteMode => {
                    write_mode = write_mode.toggle_mode();
                }
                Command::ExportFile => {
                    let res = bin_data.export_to();
                    if let Err(err) = res {
                        // エラーであれば通知する
                        let message = err.to_string();
                        notice_provider.add(Notice::new(message));
                    } else {
                        let message = format!("Saved {}", bin_data.get_file_name().into_inner());
                        notice_provider.add(Notice::new(message));
                    }
                }
                Command::InputData(value) => {
                    mini_buf.add(value);
                    dbg!(&mini_buf);
                }
                Command::MoveToUp => {
                    let index = bin_data_index.move_to_up();
                    match write_mode {
                        WriteMode::OverWrite => {
                            if let Ok(value) = bin_data.get_data(index) {
                                mini_buf.updata(value);
                            }
                        }
                        WriteMode::Insert => mini_buf.updata(HexData::new(0)),
                    }
                }
                Command::MoveToDown => {
                    let index = bin_data_index.move_to_down();
                    match write_mode {
                        WriteMode::OverWrite => {
                            if let Ok(value) = bin_data.get_data(index) {
                                mini_buf.updata(value);
                            }
                        }
                        WriteMode::Insert => mini_buf.updata(HexData::new(0)),
                    }
                }
                Command::MoveToLeft => {
                    let index = bin_data_index.move_to_left();
                    match write_mode {
                        WriteMode::OverWrite => {
                            if let Ok(value) = bin_data.get_data(index) {
                                mini_buf.updata(value);
                            }
                        }
                        WriteMode::Insert => mini_buf.updata(HexData::new(0)),
                    }
                }
                Command::MoveToRight => {
                    let index = bin_data_index.move_to_right();
                    match write_mode {
                        WriteMode::OverWrite => {
                            if let Ok(value) = bin_data.get_data(index) {
                                mini_buf.updata(value);
                            }
                        }
                        WriteMode::Insert => mini_buf.updata(HexData::new(0)),
                    }
                }
                Command::ImportFile => todo!(),
                Command::DeleteData => todo!(),
                Command::Nop => todo!(),
            }
            dbg!(&write_mode);

            let mut tui_main_panel = TuiMainPanel::new();
            tui_main_panel.set_err_msg(notice_provider.get_notice());
            tui_main_panel.set_title(bin_data.get_file_name());
            tui_main_panel.set_mode(write_mode.clone());
        }

        self.quit();
        ratatui::restore();
        // dbg!(res);

        unimplemented!();
    }
}
