// ratatui

// TUI関連
// crosstermクレート
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
// 標準ライブラリ
use std::io::{self, stdout};

// ratatuiクレート
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{block::*, *};

// 画面初期化
pub(crate) fn init_tui() -> io::Result<()> {
    // AlternateScreenへ移行
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    // raw モードに移行
    enable_raw_mode()?;

    // ratatuiウィジェット処理
    let _ = render_main();

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

pub(crate) fn render_main() -> io::Result<()> {
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
    //     "{:width$} +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F",
    //     " ",
    //     width = 8
    // ))).wrap(Wrap { trim: true }) // wrapすると先頭のスペースがトリムされてしまう
    // .block(block);

    // 16進数ヘッダー
    let header = Line::from(format!(
        "{:width$} +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F",
        " ",
        width = 8
    ));

    let contents = Paragraph::new(Text::from(vec![header])).block(block);

    // サブパネル0
    // todo!()

    // サブパネル1
    // todo!()

    let _ = terminal.draw(|frame| {
        // 左右に50%分割
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.size());

        // 右側を上下に50%分割
        let sub_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let main_panel = layout[0];
        let sub_panel_0 = sub_layout[0];
        let sub_panel_1 = sub_layout[1];

        // パネルを描画
        frame.render_widget(&contents, main_panel);
        frame.render_widget(&contents, sub_panel_0);
        frame.render_widget(&contents, sub_panel_1);
    });

    Ok(())
}
