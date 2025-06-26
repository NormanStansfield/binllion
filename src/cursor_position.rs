use crate::interfaces::MiniBufPosition;
use crate::interfaces::{CursorPosition, CursorPositionTrait};
use ratatui::layout::Position;

impl CursorPositionTrait for CursorPosition {
    fn with_mini_buf_position(
        position: CursorPosition,
        mini_buf_position: crate::interfaces::MiniBufPosition,
    ) -> CursorPosition {
        let position = if mini_buf_position == MiniBufPosition::Tail {
            let CursorPosition(Position { x, y }) = position;

            CursorPosition(Position { x: x + 1, y: y })
        } else {
            position
        };
        position
    }
}
