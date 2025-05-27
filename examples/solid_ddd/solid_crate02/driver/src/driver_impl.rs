use crate::prelude::*;

pub struct Driver;

impl DriverTrait for Driver {
    fn new() -> Self {
        Self {}
    }

    fn move_left(&self) -> Steering {
        Steering::Left
    }

    fn move_right(&self) -> Steering {
        Steering::Right
    }

    fn move_straight(&self) -> Steering {
        Steering::Straight
    }

    fn put_brake(&self) -> Velocity {
        Velocity::Deceleration
    }

    fn put_pedal(&self) -> Velocity {
        Velocity::Acceleration
    }
}
