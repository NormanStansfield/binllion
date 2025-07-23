use crate::Kilometer;
use crate::Speed;
use crate::Steering;

pub type OutputState = (Steering, Speed, Kilometer);

pub trait UiTrait {
    fn output_state(state: OutputState);
    fn output_err(message: String);
}
