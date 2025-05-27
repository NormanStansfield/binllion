use crate::Speed;
use crate::Steering;
use crate::Velocity;

pub trait CarTrait {
    fn new() -> Self;
    fn try_turn_steering_wheel(&self, steering: &Steering) -> Result<(), String>;
    fn try_pedaling(&mut self, velocity: &Velocity) -> Result<Speed, String>;
}
