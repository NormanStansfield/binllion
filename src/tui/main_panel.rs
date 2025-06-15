use ratatui::prelude::Stylize;
use ratatui::{
    symbols::border,
    text::Line,
    widgets::{Block, Borders},
};

use crate::interfaces::{FilePath, Notice, TuiArea, TuiMainPanelTrait, TuiPanelCommonTrait};
use crate::write_mode::WriteMode;

pub(crate) struct TuiMainPanel {
    message: String,
    title: String,
    mode: WriteMode,
}

impl TuiPanelCommonTrait for TuiMainPanel {
    fn new() -> Self {
        Self {
            message: String::from(""),
            title: String::from(""),
            mode: WriteMode::Insert,
        }
    }

    fn set_layout(layout: TuiArea) {
        todo!()
    }

    fn draw(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        // todo!()
        let status_bar_left =
            Line::from(vec![" Mode:".into(), self.mode.to_string().green().bold()]).left_aligned();
        let status_bar_mid = Line::from(self.message.as_ref()).centered();

        let block = Block::default()
            .title(self.title.as_ref())
            .title_bottom(status_bar_left)
            .title_bottom(status_bar_mid)
            // .title_bottom(status_bar_right)
            .borders(Borders::ALL)
            .border_set(border::THICK);
    }
}

impl TuiMainPanelTrait for TuiMainPanel {
    fn set_title(&mut self, title: Notice) {
        self.title = title.into_inner();
    }

    fn set_err_msg(&mut self, message: Notice) {
        let inner = message.into_inner();
        if inner.is_empty() {
            // self.message = String::from("");
            self.message = inner;
        } else {
            self.message = format!(" {} ", inner);
        }
    }

    fn set_mode(&mut self, mode: WriteMode) {
        // todo!()
        self.mode = mode;
    }

    fn set_content(bin_data: &[u8]) {
        todo!()
    }
}
