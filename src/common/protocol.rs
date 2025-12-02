use serde::{Serialize, Deserialize};
use crate::common::world::World;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Input(PlayerInput),
    // Join, Leave, etc.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    WorldSnapshot(World),
    // Events
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlayerInput {
    pub move_forward: bool,
    pub move_backward: bool,
    pub strafe_left: bool,
    pub strafe_right: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub shoot: bool,
    pub reload: bool,
    pub view_angle: f64,
}
