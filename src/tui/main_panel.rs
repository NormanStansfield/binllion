use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{Clear, Paragraph, Widget};
use ratatui::{
    symbols::border,
    text::Line,
    widgets::{Block, Borders},
};

use crate::constants;
use crate::interfaces::{Notice, TuiMainContentTrait, TuiMainPanelTrait};
use crate::tui::converter::{Converter, ForHex};
use crate::write_mode::WriteMode;

pub(crate) struct TuiMainPanel {
    message: String,
    title: String,
    mode: WriteMode,
}

impl TuiMainPanel {
    pub(crate) fn new() -> Self {
        Self {
            message: String::from(""),
            title: String::from(""),
            mode: WriteMode::Insert,
        }
    }
}

impl Widget for TuiMainPanel {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let status_bar_left =
            Line::from(vec![" Mode:".into(), self.mode.to_string().green().bold()]).left_aligned();
        let status_bar_mid = Line::from(self.message).centered();
        let status_bar_right =
            Line::from(vec![" Quit ".into(), "<Ctrl+Q> ".blue().bold()]).right_aligned();

        let block = Block::default()
            .title(self.title)
            .title_bottom(status_bar_left)
            .title_bottom(status_bar_mid)
            .title_bottom(status_bar_right)
            .borders(Borders::ALL)
            .border_set(border::THICK);

        Clear.render(area, buf);
        block.render(area, buf);
    }
}

impl TuiMainPanelTrait for TuiMainPanel {
    fn set_title(&mut self, title: Notice) {
        self.title = format!(" {} ", title.into_inner());
        // self.title = title.into_inner();
    }

    fn set_err_msg(&mut self, message: Notice) {
        let inner = message.into_inner();
        if inner.is_empty() {
            self.message = inner;
        } else {
            self.message = format!(" {} ", inner);
        }
    }

    fn set_mode(&mut self, mode: WriteMode) {
        self.mode = mode;
    }
}

pub(crate) struct TuiHexHeaderLabel;

impl Widget for TuiHexHeaderLabel {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // 16進数ヘッダー
        let hex_header = Line::from(format!(
            "{:width$} +0 +1 +2 +3 +4 +5 +6 +7 +8 +9 +A +B +C +D +E +F",
            " ",
            width = constants::ADDRESS_WIDTH,
        ))
        .magenta();
        hex_header.render(area, buf);
    }
}

pub(crate) struct TuiMainContent {
    content: Vec<u8>,
}

impl TuiMainContent {
    pub(crate) fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
}

impl TuiMainContentTrait for TuiMainContent {
    fn set_content(&mut self, bin_data: Vec<u8>) {
        self.content = bin_data;
    }
}

impl Widget for TuiMainContent {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // 編集データ
        let mut main_panel_data: Vec<Line<'_>> = Vec::new();
        main_panel_data.append(&mut Converter::convert_to_lines::<ForHex>(
            &self.content,
            constants::LINE_LEN,
        ));
        Paragraph::new(Text::from(main_panel_data)).render(area, buf);
    }
}
