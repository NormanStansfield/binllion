use super::prelude::*;

pub struct Car {
    speed: Speed,
    broken_info: Steering,
}

impl CarTrait for Car {
    fn new() -> Self {
        Self {
            speed: 0,
            broken_info: Steering::Left,
        }
    }

    fn try_turn_steering_wheel(&self, steering: &Steering) -> Result<(), String> {
        if *steering == self.broken_info {
            Err(format!("Cannot move to {:?}!", steering))
        } else {
            Ok(())
        }
    }

    fn try_pedaling(&mut self, velocity: &Velocity) -> Result<Speed, String> {
        let mut speed = self.speed;

        match velocity {
            Velocity::Acceleration => speed = speed.saturating_add(10),
            Velocity::Deceleration => speed = speed.saturating_sub(5),
        };

        if speed > 90 {
            speed = 90;
        }

        self.speed = speed;

        Ok(speed)
    }
}
