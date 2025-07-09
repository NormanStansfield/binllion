use crate::bin_data_index::BinDataIndex;
use crate::interfaces::{
    BinDataIndexTrait, Command, HexData, KeyEventHandlerTrait, MiniBufPosition, MiniBufTrait,
    TuiArea, TuiAsciiContentTrait as _, TuiLayoutProviderTrait, TuiMainContentTrait, ViewPosition,
    WindowTrait, WriteModeTrait,
};
use crate::key_event_handler::KeyEventHandler;
use crate::mini_buf::{self, MiniBuf};
use crate::tui::{
    self, TuiAsciiContent, TuiHexHeaderLabel, TuiMainContent, TuiMainPanel, TuiVersatilePanel,
};
use crate::tui::{TuiAsciiHeaderLabel, TuiAsciiPanel, Window};
use crate::write_mode::{self, WriteMode};
use crate::{
    bin_data::{self, BinData},
    interfaces::{
        AppTrait,
        BinDataTrait,
        FilePath,
        Notice,
        NoticeProviderTrait,
        TuiMainPanelTrait,
        // TuiPanelCommonTrait,
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
            bin_data = BinData::new();
            // 新規ファイルとしてファイルパスをセットする
            bin_data.set_path(file_path);
        }

        // 書き込みモード
        let mut write_mode = WriteMode::Insert;

        // カーソル管理
        let mut window = Window::new();

        while self.is_running() {
            bin_data_index.set_size(bin_data.get_size());

            // 領域レイアウト取得
            let layout = crate::tui::TuiLayoutProvider::get_layout(&mut self.terminal);

            // カーソル移動領域を取得
            let range = window.get_range(layout.main_content.clone());

            // メインパネル
            let mut tui_main_panel = TuiMainPanel::new();
            tui_main_panel.set_err_msg(notice_provider.get_notice());
            tui_main_panel.set_title(bin_data.get_file_name());
            tui_main_panel.set_mode(write_mode.clone());

            // メインパネル - コンテンツ
            let mut tui_main_content = TuiMainContent::new();
            let address = range.start;
            tui_main_content.set_content(bin_data.get_slice(range.clone()), address);

            // ASCIIパネル
            // let tui_ascii_panel = TuiAsciiPanel::new();
            let mut tui_ascii_content = TuiAsciiContent::new();
            tui_ascii_content.set_content(bin_data.get_slice(range), address);

            // Versatileパネル
            // let tui_versatile_panel = TuiVersatilePanel;

            // 描画
            let _ = self.terminal.draw(|frame| {
                // メインパネルを描画
                frame.render_widget(tui_main_panel, layout.main_panel.into_inner());
                frame.render_widget(TuiHexHeaderLabel, layout.main_header.into_inner());
                frame.render_widget(tui_main_content, layout.main_content.clone().into_inner());

                // ASCIIパネルを描画
                // frame.render_widget(tui_ascii_panel, layout.ascii_panel.into_inner());
                frame.render_widget(TuiAsciiPanel, layout.ascii_panel.into_inner());
                frame.render_widget(TuiAsciiHeaderLabel, layout.ascii_header.into_inner());
                frame.render_widget(tui_ascii_content, layout.ascii_content.clone().into_inner());

                // Versatileパネルを描画
                frame.render_widget(TuiVersatilePanel, layout.versatile.into_inner());
            });

            // カーソル表示
            let position = TuiMainContent::with_mini_buf_position(
                TuiMainContent::get_view_position(
                    bin_data_index.get_current_line(),
                    layout.main_content.clone(),
                    bin_data_index.get_position(),
                    window.get_window_y(),
                ),
                mini_buf.get_position(),
            );

            let ViewPosition(position) = position;
            let _ = self.terminal.set_cursor_position(position);
            let _ = self.terminal.show_cursor();

            let res = KeyEventHandler::handle_key_event();
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
                        notice_provider.add(Notice::new(message));
                    } else {
                        let message = format!("Saved {}", bin_data.get_file_name().into_inner());
                        notice_provider.add(Notice::new(message));
                    }
                }
                Command::MoveToUp => {
                    let index = bin_data_index.move_to_up();
                    window.move_to_up(bin_data_index.get_current_line());
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
                    window.move_to_down(
                        bin_data_index.get_current_line(),
                        layout.main_content.clone(),
                        bin_data_index.get_max_line(),
                    );
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
                Command::DeleteData => {
                    let index = bin_data_index.index();
                    bin_data.delete_data(index);
                }
                Command::InputData(value) => {
                    let pos = mini_buf.add(value);
                    match write_mode {
                        WriteMode::OverWrite => {
                            let res = mini_buf.to_hex();
                            if let Ok(value) = res {
                                let index = bin_data_index.index();
                                bin_data.update_data(index, value);
                            }
                        }
                        WriteMode::Insert if pos == MiniBufPosition::Head => {
                            let res = mini_buf.to_hex();
                            if let Ok(value) = res {
                                let index = bin_data_index.index();
                                bin_data.update_data(index, value);
                                mini_buf.updata(HexData::new(0));
                            }
                        }
                        WriteMode::Insert if pos == MiniBufPosition::Tail => {
                            let res = mini_buf.to_hex();
                            if let Ok(value) = res {
                                let index = bin_data_index.index();
                                bin_data.insert_data(index, value);
                            }
                        }
                        WriteMode::Insert => unreachable!(),
                    }
                    // dbg!(&mini_buf);
                }
                Command::Nop => {}
            }
            // dbg!(&write_mode);
        }

        self.quit();
        ratatui::restore();
        // dbg!(res);

        // unimplemented!();
    }
}
