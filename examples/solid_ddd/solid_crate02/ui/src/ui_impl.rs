use crate::prelude::*;

pub struct Ui;

impl UiTrait for Ui {
    fn output_state(state: OutputState) {
        println!(
            "Direction: {:?}, Speed: {}, Total Mileage: {}",
            state.0, state.1, state.2
        );
    }

    fn output_err(message: String) {
        println!("{message}");
    }
}
