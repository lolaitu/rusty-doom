use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};

pub struct Joueur {
    pub x: f64,
    pub y: f64,

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

    pub fn update(& mut self) -> Result<(),> {
        self.mouvement()?;
        Ok(())
    }

    fn mouvement(& mut self) -> Result<()> {
        let (mut dx, mut dy, mut dangle): (f64, f64, f64) = (0.0, 0.0, 0.0);
        let (mut forward, mut side): (i8, i8) = (0, 0); 

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        forward += 1;
                    }
                    KeyCode::Down => {
                        forward -= 1;
                    }
                    KeyCode::Right => {
                        side += 1;
                    }
                    KeyCode::Left => {
                        side -= 1;
                    }
                    KeyCode::w => {
                        dangle -= 1;
                    }
                    KeyCode::x => {
                        dangle += 1;
                    }
                    // Ajoutez d'autres cas selon vos besoins
                    _ => {}
                }
            }
        }

        self.angle += dangle;
        if      self.angle > 360. { self.angle -= 360.; }
        else if self.angle < 0.   { self.angle += 360.; }

        dx = (Math::)

        Ok(())
    }
}
