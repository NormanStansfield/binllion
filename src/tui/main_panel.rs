use ratatui::{
    symbols::border,
    text::Line,
    widgets::{Block, Borders},
};

use crate::interfaces::{
    FilePath, Notice, TuiArea, TuiMainPanelTrait, TuiPanelCommonTrait, WriteMode,
};

pub(crate) struct TuiMainPanel {
    message: String,
}

impl TuiPanelCommonTrait for TuiMainPanel {
    fn new() -> Self {
        Self {
            message: String::from(""),
        }
    }

    fn set_layout(layout: TuiArea) {
        todo!()
    }

    fn draw(&mut self, terminal: &mut ratatui::DefaultTerminal) {
        // todo!()
        let status_bar_mid = Line::from(self.message.as_ref()).centered();

        let block = Block::default()
            // .title(title)
            // .title_bottom(status_bar_left)
            .title_bottom(status_bar_mid)
            // .title_bottom(status_bar_right)
            .borders(Borders::ALL)
            .border_set(border::THICK);
    }
}

impl TuiMainPanelTrait for TuiMainPanel {
    fn set_title(&mut self, title: FilePath) {
        todo!()
    }

    fn set_err_msg(&mut self, message: Notice) {
        let inner = message.into_inner();
        if inner.is_empty() {
            self.message = String::from("");
        } else {
            self.message = format!(" {} ", inner);
        }
    }

    fn set_mode(&mut self, mode: WriteMode) {
        todo!()
    }

    fn set_content(bin_data: &[u8]) {
        todo!()
    }
}
