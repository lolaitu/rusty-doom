use std::io::Result;
use crate::input::InputManager;

pub trait GameMode {
    fn update(&mut self, input_manager: &InputManager) -> Result<bool>;
}
