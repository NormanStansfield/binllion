use ratatui::{
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Clear, Widget},
};

use crate::{constants, interfaces::TuiAsciiContentTrait};

pub(crate) struct TuiAsciiPanel;

impl TuiAsciiPanel {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Widget for TuiAsciiPanel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from(" ASCII ").centered())
            .borders(Borders::ALL)
            .border_set(border::THICK);

        Clear.render(area, buf);
        block.render(area, buf);
    }
}

pub(crate) struct TuiAsciiHeaderLabel;

impl Widget for TuiAsciiHeaderLabel {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // 16進数ヘッダー
        let ascii_header = Line::from(format!(
            "{:width$}+0123456789ABCDEF",
            " ",
            width = constants::ADDRESS_WIDTH
        ))
        .magenta();
        ascii_header.render(area, buf);
    }
}

pub(crate) struct TuiAsciiContent {
    content: Vec<u8>,
    address: usize,
}

impl TuiAsciiContentTrait for TuiAsciiContent {
    fn set_content(&mut self, bin_data: Vec<u8>, address: usize) {
        self.content = bin_data;
        self.address = address;
    }
}
