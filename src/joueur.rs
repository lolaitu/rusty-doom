use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};

pub struct Joueur {
    pub posX: f64,
    pub posY:f64,

    pub dirX : f64,
    pub dirY : f64,

    pub planeX : f64,
    pub planeY : f64,
}
impl Joueur {
    pub fn new() -> Result<Self> {
    let now = Instant::now();
    Ok( Self {
            posX : 1.0,
            posY : 1.0,
            
            dirX : 0.0,
            dirY : 0.0,
            planeX : 0.0,
            planeY : 0.66
        })
    }
}
