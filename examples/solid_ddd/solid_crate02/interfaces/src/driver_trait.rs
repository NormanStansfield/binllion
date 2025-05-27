use crate::Steering;
use crate::Velocity;

pub trait DriverTrait {
    fn new() -> Self;
    fn move_left(&self) -> Steering;
    fn move_right(&self) -> Steering;
    fn move_straight(&self) -> Steering;
    fn put_pedal(&self) -> Velocity;
    fn put_brake(&self) -> Velocity;
}
