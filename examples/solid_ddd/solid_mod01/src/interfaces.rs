pub trait AppTrait {
    fn new() -> Self;
    fn run(&self);
}

pub trait DriverTrait {
    fn new() -> Self;
    fn move_left(&self) -> Steering;
    fn move_right(&self) -> Steering;
    fn move_straight(&self) -> Steering;
    fn put_pedal(&self) -> Velocity;
    fn put_brake(&self) -> Velocity;
}

pub trait CarTrait {
    fn new() -> Self;
    fn try_turn_steering_wheel(&self, steering: &Steering) -> Result<(), String>;
    fn try_pedaling(&mut self, velocity: &Velocity) -> Result<Speed, String>;
}

pub type Speed = u64;
pub type Kilometer = u128;

pub enum Velocity {
    Acceleration,
    Deceleration,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Steering {
    Left,
    Right,
    #[default]
    Straight,
}

pub type OutputState = (Steering, Speed, Kilometer);

pub trait UiTrait {
    fn output_state(state: OutputState);
    fn output_err(message: String);
}
