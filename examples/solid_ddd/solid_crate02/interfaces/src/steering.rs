#[derive(Debug, Clone, Default, PartialEq)]
pub enum Steering {
    Left,
    Right,
    #[default]
    Straight,
}
