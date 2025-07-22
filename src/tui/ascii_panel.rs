use ratatui::{
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::{constants, interfaces::TuiAsciiContentTrait, tui::converter::*};

pub(crate) struct TuiAsciiPanel;

impl Widget for TuiAsciiPanel {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
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

impl TuiAsciiContent {
    pub(crate) fn new() -> Self {
        Self {
            content: Vec::<u8>::new(),
            address: 0,
        }
    }
}

impl TuiAsciiContentTrait for TuiAsciiContent {
    fn set_content(&mut self, bin_data: Vec<u8>, address: usize) {
        self.content = bin_data;
        self.address = address;
    }
}

impl Widget for TuiAsciiContent {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // Asciiデコーデッドデータ
        let mut ascii_content = Vec::new();
        ascii_content.append(&mut convert_to_lines::<ForAscii>(
            &self.content,
            self.address,
        ));
        Paragraph::new(Text::from(ascii_content)).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_tui_ascii_header_label() {
        let widget = TuiAsciiHeaderLabel;

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_tui_ascii_panel() {
        let widget = TuiAsciiPanel;

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_tui_ascii_content() {
        let mut widget = TuiAsciiContent::new();

        let mut bin_data = Vec::<u8>::new();
        let _ = (0..139).fold(0, |_acc, val| {
            bin_data.push(val);
            val
        });

        widget.set_content(bin_data, 3 * constants::LINE_LEN);

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!("001", terminal.backend());
    }
}
