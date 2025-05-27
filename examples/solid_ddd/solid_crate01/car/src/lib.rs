use interfaces::CarTrait;
use interfaces::Speed;
use interfaces::Steering;
use interfaces::Velocity;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_turn_steering_wheel() {
        let car = Car {
            speed: 0,
            broken_info: Steering::Left,
        };
        let result = car.try_turn_steering_wheel(&Steering::Left);
        assert!(result.is_err());

        // let mut car = Car {
        //     speed: 20,
        //     broken_info: Steering::Left,
        // };
        let result = car.try_turn_steering_wheel(&Steering::Right);
        assert!(result.is_ok());

        // let mut car = Car {
        //     speed: 20,
        //     broken_info: Steering::Left,
        // };
        let result = car.try_turn_steering_wheel(&Steering::Straight);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_pedaling() {
        let mut car = Car::new();
        let result = car.try_pedaling(&Velocity::Acceleration);
        assert_eq!(result.unwrap(), 10);

        let mut car = Car {
            speed: 20,
            broken_info: Steering::Left,
        };
        let result = car.try_pedaling(&Velocity::Acceleration);
        assert_eq!(result.unwrap(), 30);

        let mut car = Car {
            speed: 20,
            broken_info: Steering::Left,
        };
        let result = car.try_pedaling(&Velocity::Deceleration);
        assert_eq!(result.unwrap(), 15);

        let mut car = Car {
            speed: 300,
            broken_info: Steering::Left,
        };
        let result = car.try_pedaling(&Velocity::Deceleration);
        assert_eq!(result.unwrap(), 90);
    }
}
