use crate::interfaces::AppTrait;
use crate::interfaces::CarTrait as _;
use crate::interfaces::DriverTrait as _;
use crate::interfaces::Steering;
use crate::interfaces::UiTrait as _;

use crate::car::Car;
use crate::driver::Driver;
use crate::ui::Ui;

mod state;
use state::State;

pub struct App {}

impl AppTrait for App {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) {
        let driver = Driver::new();
        let mut car = Car::new();
        let mut state = State::default();

        let vel = driver.put_pedal();
        let res = car.try_pedaling(&vel);
        match res {
            Ok(speed) => {
                state.set_speed(speed);
            }
            Err(err) => {
                Ui::output_err(err);
            }
        };

        let direction = driver.move_left();
        let res = car.try_turn_steering_wheel(&direction);
        if let Err(err) = res {
            Ui::output_err(err);
        } else {
            state.set_steering(direction);
            state.update_mileage();
        }

        Ui::output_state(state.to_output());

        (1..20).for_each(|_| {
            let direction = App::get_direction(&driver);
            let vel = App::get_pedaling(&driver);

            let res = car.try_pedaling(&vel);
            match res {
                Ok(speed) => {
                    state.set_speed(speed);
                }
                Err(err) => {
                    Ui::output_err(err);
                }
            };

            let res = car.try_turn_steering_wheel(&direction);
            if let Err(err) = res {
                Ui::output_err(err);
            } else {
                state.set_steering(direction);
                state.update_mileage();
            }

            Ui::output_state(state.to_output());
        });
    }
}

use crate::interfaces::Velocity;
use rand::Rng;
impl App {
    fn get_direction(driver: &Driver) -> Steering {
        let mut rng = rand::rng();

        let n: u8 = rng.random_range(0..=100);

        match n {
            0..=40 => driver.move_left(),
            41..=80 => driver.move_right(),
            81..=100 => driver.move_straight(),
            _ => unreachable!(),
        }
    }

    fn get_pedaling(driver: &Driver) -> Velocity {
        let mut rng = rand::rng();

        let n: u8 = rng.random_range(0..=100);

        match n {
            0..=50 => driver.put_brake(),
            51..=100 => driver.put_pedal(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::App;
    use crate::driver::Driver;
    #[allow(unused_imports)]
    use crate::interfaces::*;

    #[test]
    fn test_get_direction() {
        let driver = Driver::new();

        let mut count_right = 0;
        let mut count_left = 0;
        let mut count_straight = 0;

        (1..200).for_each(|_| {
            let res = App::get_direction(&driver);

            match res {
                Steering::Right => {
                    count_right += 1;
                }
                Steering::Left => {
                    count_left += 1;
                }
                Steering::Straight => {
                    count_straight += 1;
                }
            }
        });
        assert!(
            count_right > 0 && count_left > 0 && count_straight > 0,
            "right {} : left {} : straight{}",
            count_right,
            count_left,
            count_straight
        );
    }

    #[test]
    fn test_get_pedaling() {
        let driver = Driver::new();

        let mut count_acceleration = 0;
        let mut count_deceleration = 0;

        (1..200).for_each(|_| {
            let res = App::get_pedaling(&driver);

            match res {
                Velocity::Acceleration => {
                    count_acceleration += 1;
                }
                Velocity::Deceleration => {
                    count_deceleration += 1;
                }
            }
        });
        assert!(
            count_acceleration > 0 && count_deceleration > 0,
            "acceleration {} : deceleration {}",
            count_acceleration,
            count_deceleration,
        );
    }
}
