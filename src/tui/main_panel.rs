use ratatui::layout::{Position, Rect};
use ratatui::prelude::Stylize;
use ratatui::text::Text;
use ratatui::widgets::{Clear, Paragraph, Widget};
use ratatui::{
    symbols::border,
    text::Line,
    widgets::{Block, Borders},
};

use crate::constants;
use crate::interfaces::{
    MiniBufPosition, Notice, TuiArea, TuiMainContentTrait, TuiMainPanelTrait, ViewPosition,
};
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
    address: usize,
}

impl TuiMainContent {
    pub(crate) fn new() -> Self {
        Self {
            content: Vec::new(),
            address: 0,
        }
    }
}

impl TuiMainContentTrait for TuiMainContent {
    fn set_content(&mut self, bin_data: Vec<u8>, address: usize) {
        self.content = bin_data;
        self.address = address;
    }

    fn with_mini_buf_position(
        position: ViewPosition,
        mini_buf_position: crate::interfaces::MiniBufPosition,
    ) -> ViewPosition {
        let position = if mini_buf_position == MiniBufPosition::Left {
            let ViewPosition(Position { x, y }) = position;

            ViewPosition(Position::new(x - 1, y))
        } else {
            position
        };
        position
    }

    fn get_view_position(
        current_line: usize,
        area: TuiArea,
        position: ViewPosition,
        window_y: usize,
    ) -> ViewPosition {
        let area = area.into_inner();
        let ViewPosition(position) = position;

        let x = area.x + constants::ADDRESS_WIDTH as u16 + 2 + position.x * constants::STEP as u16;
        let y = area.y + (current_line.saturating_sub(window_y)) as u16;

        ViewPosition(Position::new(x, y))
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
            self.address,
        ));
        Paragraph::new(Text::from(main_panel_data)).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_tui_hex_header_label() {
        let widget = TuiHexHeaderLabel;

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_tui_main_panel_001() {
        let mut widget = TuiMainPanel::new();

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!("001", terminal.backend());
    }

    #[test]
    fn test_tui_main_panel_002() {
        let mut widget = TuiMainPanel::new();
        widget.set_title(Notice::new("test_title".to_string()));

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!("002", terminal.backend());
    }

    #[test]
    fn test_tui_main_panel_003() {
        let mut widget = TuiMainPanel::new();
        widget.set_mode(WriteMode::OverWrite);

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!("003", terminal.backend());
    }

    #[test]
    fn test_tui_main_panel_004() {
        let mut widget = TuiMainPanel::new();
        widget.set_err_msg(Notice::new("Test Message".to_string()));

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(widget, frame.area()))
            .unwrap();
        assert_snapshot!("004", terminal.backend());
    }

    #[test]
    fn test_tui_main_content() {
        let mut widget = TuiMainContent::new();

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
        assert_snapshot!(terminal.backend());

        let view_position = ViewPosition(Position { x: 10, y: 10 });
        let mini_buf_position = MiniBufPosition::Right;

        let res = TuiMainContent::with_mini_buf_position(view_position, mini_buf_position);
        assert_eq!(res, ViewPosition(Position { x: 10, y: 10 }));

        let view_position = ViewPosition(Position { x: 10, y: 10 });
        let mini_buf_position = MiniBufPosition::Left;

        let res = TuiMainContent::with_mini_buf_position(view_position, mini_buf_position);
        assert_eq!(res, ViewPosition(Position { x: 9, y: 10 }));

        let current_line = 15;
        let position = ViewPosition(Position { x: 10, y: 10 });
        let area = TuiArea::new(Rect {
            x: 5,
            y: 5,
            width: 55,
            height: 55,
        });
        let window_y = 0;

        let res = TuiMainContent::get_view_position(current_line, area.clone(), position, window_y);
        assert_eq!(res, ViewPosition(Position { x: 45, y: 20 }));

        let current_line = 15;
        let position = ViewPosition(Position { x: 15, y: 15 });
        let window_y = 0;

        let res = TuiMainContent::get_view_position(current_line, area.clone(), position, window_y);
        assert_eq!(res, ViewPosition(Position { x: 60, y: 20 }));

        let current_line = 25;
        let position = ViewPosition(Position { x: 15, y: 15 });
        let window_y = 20;

        let res = TuiMainContent::get_view_position(current_line, area.clone(), position, window_y);
        assert_eq!(res, ViewPosition(Position { x: 60, y: 10 }));
    }
}
