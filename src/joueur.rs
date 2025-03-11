use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};

pub struct Joueur {
    pub x: f64,
    pub y:f64,

    pub angle: f64,
}
impl Joueur {
    pub fn new() -> Result<Self> {
    let now = Instant::now();
    Ok( Self {
            x : 4.0,
            y : 11.0,
            angle: 0.0,
        })
    }

    pub fn update(& mut self) -> Result<()> {
        self.angle += 1.;
        if (self.angle > 360.) { self.angle = 0.; }
        Ok(())
    }
}
