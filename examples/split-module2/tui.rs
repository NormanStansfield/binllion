// split-module2

// TUI関連
// crosstermの読み込み
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
// 標準ライブラリの読み込み
use std::io::{self, stdout};

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
