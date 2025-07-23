use crate::prelude::*;

#[derive(Default)]
pub(crate) struct State {
    steering: Steering,
    speed: Speed,
    mileage: Kilometer,
}

impl State {
    pub(crate) fn set_steering(&mut self, steering: Steering) {
        self.steering = steering;
    }

    pub(crate) fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    pub(crate) fn update_mileage(&mut self) {
        self.mileage += self.speed as Kilometer;
    }
}

impl State {
    pub(crate) fn to_output(&self) -> OutputState {
        let output: OutputState = (self.steering.clone(), self.speed, self.mileage);
        output
    }
}
