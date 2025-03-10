// TUI関連

// モジュールファイルの読み込み
mod converter;

// 標準ライブラリ
use std::io::{self};
// ratatuiクレート
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::DefaultTerminal;
// 状態管理
use crate::message::{Message, Scroll};
// 変換処理系
use crate::tui::converter::{Converter, ForAscii, ForHex};
// 定数
use crate::constants;

// 画面初期化
// pub(crate) fn init_tui() -> io::Result<()> {
//     // AlternateScreenへ移行
//     crossterm::execute!(stdout(), EnterAlternateScreen)?;
//     // raw モードに移行
//     enable_raw_mode()?;

//     Ok(())
// }
// 画面復旧
// pub(crate) fn end_tui() -> io::Result<()> {
//     // raw モードを解除
//     disable_raw_mode()?;
//     // AlternateScreenから復帰
//     crossterm::execute!(stdout(), LeaveAlternateScreen)?;

//     Ok(())
// }
// ratatuiウィジェットレンダリング
pub(crate) fn render_main(terminal: &mut DefaultTerminal, message: &Message) -> io::Result<()> {
    let bin_data = message.bin_data();
    let cursor = message.cursor();
    let layout = message.layout();

    // メインパネル
    // 上タイトル

    // ファイル名のみを取得してタイトルとする
    let file_name = message.current_file().file_name();
    let title = Line::from(format!(" {file_name} ").bold()).centered();

    // 下タイトル(ステータスバー)
    let mode = {
        use crate::message::WriteMode::*;
        match message.write_mode() {
            OverWrite => " OVR ",
            Insert => " INT ",
        }
    };
    let status_bar_left = Line::from(vec![" Mode:".into(), mode.green().bold()]).left_aligned();
    let status_bar_mid = Line::from("").centered();
    let status_bar_right =
        Line::from(vec![" Quit ".into(), "<Ctrl+Q> ".blue().bold()]).right_aligned();

    // パネルブロック
    let block = Block::default()
        .title(title)
        .title_bottom(status_bar_left)
        .title_bottom(status_bar_mid)
        .title_bottom(status_bar_right)
        .borders(Borders::ALL)
        .border_set(border::THICK);

    // let header = Paragraph::new(Text::from(format!(
    //     "{:width$} +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F",
    //     " ",
    //     width = 8
    // ))).wrap(Wrap { trim: true }) // wrapすると先頭のスペースがトリムされてしまう
    // .block(block);

    // メインパネル

    // 16進数ヘッダー
    let hex_header = Line::from(format!(
        "{:width$} +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F",
        " ",
        width = 8
    ))
    .magenta();

    let mut main_panel_data = Vec::new();
    // main_panel_data.push(hex_header);
    main_panel_data.append(&mut Converter::convert_to_lines::<ForHex>(
        bin_data.buf(),
        constants::LINE_LEN,
    ));

    let main_contents =
        Paragraph::new(Text::from(main_panel_data)).scroll((message.scroll().scroll_y()[0], 0));
    // .block(block.clone());

    // サブパネル0

    // Asciiヘッダー
    let ascii_header = Line::from(format!("{:width$}+0123456789ABCDEF", " ", width = 8)).magenta();

    let mut sub0_panel_data = Vec::new();
    // sub0_panel_data.push(ascii_header);
    sub0_panel_data.append(&mut Converter::convert_to_lines::<ForAscii>(
        bin_data.buf(),
        constants::LINE_LEN,
    ));

    let sub0_contents =
        Paragraph::new(Text::from(sub0_panel_data)).scroll((message.scroll().scroll_y()[1], 0));
    // .block(block.clone());

    // サブパネル1
    // todo!()

    let _ = terminal.draw(|frame| {
        let main_panel = layout[0][0];
        let main_header = layout[2][0];
        let main_area = layout[2][1];

        let sub0_panel = layout[1][0];
        let sub0_header = layout[3][0];
        let sub0_area = layout[3][1];

        let sub1_panel = layout[1][1];

        // パネルを描画
        frame.render_widget(Clear, frame.area());

        frame.render_widget(&block, main_panel);
        frame.render_widget(hex_header, main_header);
        frame.render_widget(main_contents, main_area);

        frame.render_widget(&block, sub0_panel);
        frame.render_widget(ascii_header, sub0_header);
        frame.render_widget(sub0_contents, sub0_area);

        frame.render_widget(&block, sub1_panel);
    });

    // カーソル表示
    let _ = terminal.set_cursor_position(*cursor.position());
    let _ = terminal.show_cursor();

    Ok(())
}
// ratatuiレンダリング準備
pub(crate) fn render_prep(terminal: &mut DefaultTerminal, message: &mut Message) -> io::Result<()> {
    let frame = terminal.get_frame();

    // 左右に50%分割
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    // 右側を上下に50%分割
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    // 左側をヘッダーとコンテンツに分割
    let inner_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .margin(1)
        .split(main_layout[0]);

    // 右上側をヘッダーとコンテンツに分割
    let inner_sub0 = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .margin(1)
        .split(sub_layout[0]);

    // カーソルY座標の算出
    let pos_y = {
        let cursor = message.cursor_mut();
        cursor.calc_position();
        cursor.position().y
    };
    // 画面の下限
    let main_bottom = main_layout[0].bottom();
    // メインパネルのスクロール開始ラインの算出
    let main_border = Scroll::calc_border(main_bottom);
    // メインパネルのスクロール量計算
    let scroll_y = message.scroll_mut().scroll_y_mut();
    scroll_y[0] = Scroll::calc_scroll(pos_y, main_border);

    // サブパネルの下限
    let sub0_bottom = sub_layout[0].bottom();
    // サブ0パネルのスクロール開始ラインの算出
    let sub0_border = Scroll::calc_border(sub0_bottom);
    // サブ0パネルのスクロール量計算
    scroll_y[1] = Scroll::calc_scroll(pos_y, sub0_border);

    // カーソルY座標の調整
    let cursor = message.cursor_mut();
    cursor.adjust_y(main_border);

    // レイアウトの保存
    let screen_layout = message.layout_mut();
    screen_layout[0] = main_layout;
    screen_layout[1] = sub_layout;
    screen_layout[2] = inner_main;
    screen_layout[3] = inner_sub0;

    Ok(())
}
