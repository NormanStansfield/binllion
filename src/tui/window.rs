use crate::constants;
use crate::interfaces::TuiArea;
use crate::interfaces::WindowTrait;
use std::ops::Range;

pub(crate) struct Window {
    origin_wy: usize,
}

impl WindowTrait for Window {
    fn window_range(&self, area: TuiArea) -> Range<usize> {
        let area = area.into_inner();

        let upper_line = self.origin_wy;
        let bottom_line = self.origin_wy + (area.height - area.y) as usize;

        let begin_line: usize = upper_line * constants::LINE_LEN;
        let end_line: usize = bottom_line * constants::LINE_LEN + constants::LINE_LEN;

        begin_line..end_line
    }

    fn new() -> Self {
        Self { origin_wy: 0 }
    }

    fn move_to_up(&mut self, current_line: usize) {
        if current_line < self.origin_wy {
            self.origin_wy = current_line;
        }
    }

    fn move_to_down(&mut self, current_line: usize, area: TuiArea, max_line: usize) {
        let area = area.into_inner();
        let window_y_height = (area.height - area.y) as usize;
        if current_line > self.origin_wy + window_y_height {
            self.origin_wy = current_line - window_y_height;
        }
        if self.origin_wy + window_y_height > max_line {
            self.origin_wy = max_line.saturating_sub(window_y_height);
        }
    }
}

impl Window {
    pub(crate) fn origin_wy(&self) -> usize {
        self.origin_wy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn test_window() {
        let mut window = Window::new();
        let current_line = 5;
        let max_line = 40;
        let area = TuiArea::new(Rect {
            x: 10,
            y: 10,
            width: 30,
            height: 30,
        });

        assert_eq!(window.origin_wy(), 0);

        window.move_to_down(current_line, area.clone(), max_line);
        assert_eq!(window.origin_wy(), 0);
        assert_eq!(
            window.window_range(area.clone()),
            0 * constants::LINE_LEN..20 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 20;
        window.move_to_down(current_line, area.clone(), max_line);
        assert_eq!(window.origin_wy(), 0);
        assert_eq!(
            window.window_range(area.clone()),
            0 * constants::LINE_LEN..20 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 21;
        window.move_to_down(current_line, area.clone(), max_line);
        assert_eq!(window.origin_wy(), 1);
        assert_eq!(
            window.window_range(area.clone()),
            1 * constants::LINE_LEN..21 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 22;
        window.move_to_down(current_line, area.clone(), max_line);
        assert_eq!(window.origin_wy(), 2);
        assert_eq!(
            window.window_range(area.clone()),
            2 * constants::LINE_LEN..22 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 35;
        window.move_to_down(current_line, area.clone(), max_line);
        assert_eq!(window.origin_wy(), 15);
        assert_eq!(
            window.window_range(area.clone()),
            15 * constants::LINE_LEN..35 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 40;
        window.move_to_down(current_line, area.clone(), max_line);
        assert_eq!(window.origin_wy(), 20);
        assert_eq!(
            window.window_range(area.clone()),
            20 * constants::LINE_LEN..40 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 41;
        window.move_to_down(current_line, area.clone(), max_line);
        assert_eq!(window.origin_wy(), 20);
        assert_eq!(
            window.window_range(area.clone()),
            20 * constants::LINE_LEN..40 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 39;
        window.move_to_up(current_line);
        assert_eq!(window.origin_wy(), 20);
        assert_eq!(
            window.window_range(area.clone()),
            20 * constants::LINE_LEN..40 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 20;
        window.move_to_up(current_line);
        assert_eq!(window.origin_wy(), 20);
        assert_eq!(
            window.window_range(area.clone()),
            20 * constants::LINE_LEN..40 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 19;
        window.move_to_up(current_line);
        assert_eq!(window.origin_wy(), 19);
        assert_eq!(
            window.window_range(area.clone()),
            19 * constants::LINE_LEN..39 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 10;
        window.move_to_up(current_line);
        assert_eq!(window.origin_wy(), 10);
        assert_eq!(
            window.window_range(area.clone()),
            10 * constants::LINE_LEN..30 * constants::LINE_LEN + constants::LINE_LEN
        );

        let current_line = 0;
        window.move_to_up(current_line);
        assert_eq!(window.origin_wy(), 0);
        assert_eq!(
            window.window_range(area.clone()),
            0 * constants::LINE_LEN..20 * constants::LINE_LEN + constants::LINE_LEN
        );
    }
}
