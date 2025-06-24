use crate::interfaces::{TuiArea, TuiLayout, TuiLayoutProviderTrait};

pub(crate) struct TuiLayoutProvider;

impl TuiLayoutProviderTrait for TuiLayoutProvider {
    fn get_layout(terminal: &mut ratatui::DefaultTerminal) -> TuiLayout {
        let frame = terminal.get_frame();

        // 左右に50%分割
        let main_layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![
                ratatui::layout::Constraint::Percentage(50),
                ratatui::layout::Constraint::Percentage(50),
            ])
            .split(frame.area());

        // 右側を上下に50%分割
        let sub_layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Percentage(50),
                ratatui::layout::Constraint::Percentage(50),
            ])
            .split(main_layout[1]);

        // 左側をヘッダーとコンテンツに分割
        let inner_main = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(1),
                ratatui::layout::Constraint::Min(1),
            ])
            .margin(1)
            .split(main_layout[0]);

        // 右上側をヘッダーとコンテンツに分割
        let inner_sub0 = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(1),
                ratatui::layout::Constraint::Min(1),
            ])
            .margin(1)
            .split(sub_layout[0]);

        // レイアウトの保存
        let tui_layout = TuiLayout {
            main_panel: TuiArea::new(main_layout[0]),
            main_header: TuiArea::new(inner_main[0]),
            main_content: TuiArea::new(inner_main[1]),
            ascii_panel: TuiArea::new(sub_layout[0]),
            ascii_header: TuiArea::new(inner_sub0[0]),
            ascii_content: TuiArea::new(inner_sub0[1]),
            versatile: TuiArea::new(sub_layout[1]),
        };
        tui_layout
    }
}
