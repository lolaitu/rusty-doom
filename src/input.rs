use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use device_query::{DeviceQuery, DeviceState, Keycode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveForward,
    MoveBackward,
    StrafeLeft,
    StrafeRight,
    RotateLeft,
    RotateRight,
    Shoot,
    Reload,
    RespawnEnemies,
    RespawnPlayer,
    Quit,
    None,
}

pub struct InputManager {
    device_state: DeviceState,
    active_actions: HashSet<Action>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            active_actions: HashSet::new(),
        }
    }

    pub fn update(&mut self) {
        let keys: Vec<Keycode> = self.device_state.get_keys();
        
        self.active_actions.clear();

        // Map keys to actions
        if keys.contains(&Keycode::Up) || keys.contains(&Keycode::W) {
            self.active_actions.insert(Action::MoveForward);
        }
        if keys.contains(&Keycode::Down) || keys.contains(&Keycode::S) {
            self.active_actions.insert(Action::MoveBackward);
        }
        if keys.contains(&Keycode::Left) || keys.contains(&Keycode::A) {
            self.active_actions.insert(Action::StrafeLeft);
        }
        if keys.contains(&Keycode::Right) || keys.contains(&Keycode::D) {
            self.active_actions.insert(Action::StrafeRight);
        }
        
        // Rotation
        if keys.contains(&Keycode::Q) || keys.contains(&Keycode::X) {
            self.active_actions.insert(Action::RotateLeft);
        }
        if keys.contains(&Keycode::E) {
            self.active_actions.insert(Action::RotateRight);
        }

        // Actions
        if keys.contains(&Keycode::Space) {
            self.active_actions.insert(Action::Shoot);
        }
        if keys.contains(&Keycode::R) {
            self.active_actions.insert(Action::Reload);
        }
        if keys.contains(&Keycode::P) {
            self.active_actions.insert(Action::RespawnEnemies);
        }

        // Quit: Ctrl + C
        // Check for either left or right control
        let ctrl_pressed = keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl);
        if ctrl_pressed && keys.contains(&Keycode::C) {
            self.active_actions.insert(Action::Quit);
        }
    }

    pub fn is_active(&self, action: Action) -> bool {
        self.active_actions.contains(&action)
    }
}
