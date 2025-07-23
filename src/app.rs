use crate::interfaces::AppTrait;
use crate::interfaces::BinDataIndexTrait as _;
use crate::interfaces::BinDataTrait as _;
use crate::interfaces::CurrentFileTrait as _;
use crate::interfaces::KeyEventHandlerTrait as _;
use crate::interfaces::MiniBufTrait as _;
use crate::interfaces::NoticeProviderTrait as _;
use crate::interfaces::TuiAsciiContentTrait as _;
use crate::interfaces::TuiLayoutProviderTrait as _;
use crate::interfaces::TuiMainContentTrait as _;
use crate::interfaces::TuiMainPanelTrait as _;
use crate::interfaces::WindowTrait as _;
use crate::interfaces::WriteModeTrait as _;

use crate::interfaces::Command;
use crate::interfaces::HexData;
use crate::interfaces::MiniBufPosition;
use crate::interfaces::ViewPosition;

use crate::bin_data::BinData;
use crate::bin_data_index::BinDataIndex;
use crate::interfaces::FilePath;
use crate::interfaces::Notice;
use crate::key_event_handler::KeyEventHandler;
use crate::mini_buf::MiniBuf;
use crate::notice_provider::NoticeProvider;
use crate::write_mode::WriteMode;

use crate::tui::{
    TuiAsciiContent, TuiAsciiHeaderLabel, TuiAsciiPanel, TuiHexHeaderLabel, TuiMainContent,
    TuiMainPanel, TuiVersatilePanel, Window,
};

pub(crate) struct App {
    // 描画用
    terminal: ratatui::DefaultTerminal,
    // ループが継続中フラグ
    running: bool,
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
            notice_provider.push(Notice::new(message));
            bin_data = BinData::new();
            // 新規ファイルとしてファイルパスをセットする
            bin_data.set_path(file_path);
        }

        // 書き込みモード
        let mut write_mode = WriteMode::Insert;

        // カーソル管理
        let mut window = Window::new();

        while self.is_running() {
            bin_data_index.set_size(bin_data.len());

            // 領域レイアウト取得
            let layout = crate::tui::TuiLayoutProvider::layout(&mut self.terminal);

            // カーソル移動領域を取得
            let range = window.window_range(layout.main_content.clone());

            // メインパネル
            let mut tui_main_panel = TuiMainPanel::new();
            tui_main_panel.set_err_msg(notice_provider.pop());
            tui_main_panel.set_title(bin_data.file_name());
            tui_main_panel.set_mode(write_mode.clone());

            // メインパネル - コンテンツ
            let mut tui_main_content = TuiMainContent::new();
            let address = range.start;
            tui_main_content.set_content(bin_data.to_vec(range.clone()), address);

            // ASCIIパネル
            let mut tui_ascii_content = TuiAsciiContent::new();
            tui_ascii_content.set_content(bin_data.to_vec(range), address);

            // Versatileパネル
            // let tui_versatile_panel = TuiVersatilePanel;

            // 描画
            let _ = self.terminal.draw(|frame| {
                // メインパネルを描画
                frame.render_widget(tui_main_panel, layout.main_panel.into_inner());
                frame.render_widget(TuiHexHeaderLabel, layout.main_header.into_inner());
                frame.render_widget(tui_main_content, layout.main_content.clone().into_inner());

                // ASCIIパネルを描画
                frame.render_widget(TuiAsciiPanel, layout.ascii_panel.into_inner());
                frame.render_widget(TuiAsciiHeaderLabel, layout.ascii_header.into_inner());
                frame.render_widget(tui_ascii_content, layout.ascii_content.clone().into_inner());

                // Versatileパネルを描画
                frame.render_widget(TuiVersatilePanel, layout.versatile.into_inner());
            });

            // カーソル表示
            let position = TuiMainContent::with_mini_buf_position(
                TuiMainContent::view_position(
                    bin_data_index.current_line(),
                    layout.main_content.clone(),
                    bin_data_index.position(),
                    window.origin_wy(),
                ),
                mini_buf.mini_buf_position(),
            );

            let ViewPosition(position) = position;
            let _ = self.terminal.set_cursor_position(position);
            let _ = self.terminal.show_cursor();

            let res = KeyEventHandler::handle_key_event(crossterm::event::read());
            // dbg!(&res);

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
                        notice_provider.push(Notice::new(message));
                    } else {
                        let message = format!("Saved {}", bin_data.file_name().into_inner());
                        notice_provider.push(Notice::new(message));
                    }
                }
                Command::MoveToUp => {
                    let index = bin_data_index.move_to_up();
                    window.move_to_up(bin_data_index.current_line());
                    match write_mode {
                        WriteMode::OverWrite => {
                            if let Some(value) = bin_data.to_hex_data(index) {
                                mini_buf.updata(value);
                            }
                        }
                        WriteMode::Insert => mini_buf.updata(HexData::new(0)),
                    }
                }
                Command::MoveToDown => {
                    let index = bin_data_index.move_to_down();
                    window.move_to_down(
                        bin_data_index.current_line(),
                        layout.main_content.clone(),
                        bin_data_index.max_line(),
                    );
                    match write_mode {
                        WriteMode::OverWrite => {
                            if let Some(value) = bin_data.to_hex_data(index) {
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
                            if let Some(value) = bin_data.to_hex_data(index) {
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
                            if let Some(value) = bin_data.to_hex_data(index) {
                                mini_buf.updata(value);
                            }
                        }
                        WriteMode::Insert => mini_buf.updata(HexData::new(0)),
                    }
                }
                // Command::ImportFile => unimplemented!(),
                Command::DeleteData => {
                    let index = bin_data_index.index();
                    bin_data.delete_data(index);
                }
                Command::InputData(value) => {
                    let pos = mini_buf.add(value);
                    match write_mode {
                        WriteMode::OverWrite => {
                            let res = mini_buf.to_hex_data();
                            if let Ok(value) = res {
                                let index = bin_data_index.index();
                                bin_data.update_data(index, value);
                            }
                        }
                        WriteMode::Insert if pos == MiniBufPosition::Right => {
                            let res = mini_buf.to_hex_data();
                            if let Ok(value) = res {
                                let index = bin_data_index.index();
                                bin_data.update_data(index, value);
                                mini_buf.updata(HexData::new(0));
                            }
                        }
                        WriteMode::Insert if pos == MiniBufPosition::Left => {
                            let res = mini_buf.to_hex_data();
                            if let Ok(value) = res {
                                let index = bin_data_index.index();
                                bin_data.insert_data(index, value);
                            }
                        }
                        WriteMode::Insert => unreachable!(),
                    }
                }
                Command::Nop => {}
            }
        }

        self.quit();
        ratatui::restore();
    }
}
