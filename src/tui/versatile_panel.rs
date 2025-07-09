use ratatui::{
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Clear, Widget},
};

pub(crate) struct TuiVersatilePanel;

impl Widget for TuiVersatilePanel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(Line::from("").centered())
            .borders(Borders::ALL)
            .border_set(border::THICK);

        Clear.render(area, buf);
        block.render(area, buf);
    }
}
