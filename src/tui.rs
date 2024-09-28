// ratatui

// TUI関連
// crosstermクレート
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
// 標準ライブラリ
use std::io::{self, stdout};
use std::collections::VecDeque;

// ratatuiクレート
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::*;

// 編集用構造体
struct BinData {
    buf: VecDeque<u8>,
}

impl BinData {
    pub(crate) fn new() -> Self {
        BinData { buf: VecDeque::new() }
    }

    pub(crate) fn push_back(&mut self, new_buf: Vec<u8>) {
        let mut new_data: VecDeque<u8> = VecDeque::from(new_buf);
        self.buf.make_contiguous();
        self.buf.append(&mut new_data);
    }

    pub(crate) fn insert(&mut self, index: usize, value: u8) {
        self.buf.make_contiguous();
        self.buf.insert(index, value);
    }

    pub(crate) fn update(&mut self, index: usize, value: u8) {
        self.buf.make_contiguous();
        if let Some(elem) = self.buf.get_mut(index) {
            *elem = value;
        }
    }

    pub(crate) fn buf(&self) -> &[u8] {
        let (res, _) = self.buf.as_slices();
        res
    }

}

impl From<Vec<u8>> for BinData {
    fn from(buf: Vec<u8>) -> Self {
        BinData { buf: VecDeque::from(buf) }
    }
}

impl Default for BinData {
    fn default() -> Self {
        Self::new()
    }
}

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
        "{:width$} +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F",
        " ",
        width = 8
    ));

    // 仮データ
    let mut bin_data = BinData::new();
    bin_data.push_back(vec![
        0x01, 0x02, 0x03, 0x00, 0x63, 0x71, 0x00, 0x61, 0x62, 0x0f, 0x01, 0x02, 0x03, 0x00, 0x63,
        0x71, 0x0f, 0x61, 0x62, 0x63, 0x01, 0xff, 0x03, 0x00, 0x63, 0x71, 0x0f, 0x61, 0x62, 0x63,
    ]);

    // 表示データ作成
    let mut main_panel_data = Vec::new();
    main_panel_data.push(header);
    main_panel_data.append(&mut self::to_lines(bin_data.buf(), 8));

    let contents = Paragraph::new(Text::from(main_panel_data)).block(block);

    // サブパネル0
    // todo!()

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
        frame.render_widget(&contents, main_panel);
        frame.render_widget(&contents, sub_panel_0);
        frame.render_widget(&contents, sub_panel_1);
    });

    Ok(())
}

pub(crate) fn to_hex(buf: &[u8]) -> String {
    let sep = String::from(" ");
    let hex = buf.iter().map(|x| format!("{:02X}", x)).collect::<Vec<_>>().join(&sep);
    // dbg!(&ret);
    hex
}

pub(crate) fn to_lines(buf: &[u8], len: usize) -> Vec<Line> {
    let mut vec = Vec::new();
    buf.chunks(len).for_each(|x| vec.push(Line::from(format!("{:width$} {}", " ", self::to_hex(x), width = 8))));
    // dbg!(&vec);
    vec
}