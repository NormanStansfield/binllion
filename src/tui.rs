// TUI関連

// モジュールファイルの読み込み
mod converter;

// crosstermクレート
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
// 標準ライブラリ
use std::io::{self, stdout};
// ratatuiクレート
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::*;
// 状態管理
use crate::message::Message;
// 変換処理系
use crate::tui::converter::{Converter, ForAscii, ForHex};
// 定数
use crate::constants;

// 画面初期化
pub(crate) fn init_tui() -> io::Result<()> {
    // AlternateScreenへ移行
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    // raw モードに移行
    enable_raw_mode()?;

    Ok(())
}
// 画面復旧
pub(crate) fn end_tui() -> io::Result<()> {
    // raw モードを解除
    disable_raw_mode()?;
    // AlternateScreenから復帰
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
// ratatuiウィジェットレンダリング
pub(crate) fn render_main(message: &Message) -> io::Result<()> {
    let bin_data = message.bin_data();
    let cursor = message.cursor();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // メインパネル
    // 上タイトル
    let title = Title::from(" main ".bold());
    // 下タイトル
    let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Ctrl+Q> ".blue().bold()]));

    // パネルブロック
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
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
    let header = Line::from(format!(
        "{:width$} +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F",
        " ",
        width = 8
    ));

    let mut main_panel_data = Vec::new();
    main_panel_data.push(header);
    main_panel_data.append(&mut Converter::convert_to_lines::<ForHex>(
        bin_data.buf(),
        constants::LINE_LEN,
    ));

    let main_contents = Paragraph::new(Text::from(main_panel_data))
        .scroll((*cursor.scroll_y(), 0))
        .block(block.clone());

    // サブパネル0

    // Asciiヘッダー
    let header = Line::from(format!("{:width$}+0123456789ABCDEF", " ", width = 8));

    let mut sub0_panel_data = Vec::new();
    sub0_panel_data.push(header);
    sub0_panel_data.append(&mut Converter::convert_to_lines::<ForAscii>(
        bin_data.buf(),
        constants::LINE_LEN,
    ));

    let sub0_contents = Paragraph::new(Text::from(sub0_panel_data))
        .scroll((*cursor.scroll_y(), 0))
        .block(block.clone());

    // サブパネル1
    // todo!()

    let _ = terminal.draw(|frame| {
        // 左右に50%分割
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        // 右側を上下に50%分割
        let sub_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let main_panel = layout[0];
        let sub_panel_0 = sub_layout[0];
        let sub_panel_1 = sub_layout[1];

        // パネルを描画
        frame.render_widget(&main_contents, main_panel);
        frame.render_widget(&sub0_contents, sub_panel_0);
        frame.render_widget(&sub0_contents, sub_panel_1);
    });

    // カーソル表示
    let _ = terminal.set_cursor_position(*cursor.position());
    let _ = terminal.show_cursor();

    Ok(())
}
// ratatuiレンダリング準備
pub(crate) fn render_prep(message: &mut Message) -> io::Result<()> {
    // let bin_data = message.bin_data_mut();
    let cursor = message.cursor_mut();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let bottom = terminal.get_frame().area().bottom();
    cursor.calc_scroll(bottom);

    Ok(())
}
