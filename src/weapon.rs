use std::time::{Duration, Instant};
use crossterm::style::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponType {
    Pistol,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponState {
    Idle,
    Firing,
    Recoil,
    Reloading,
}

pub struct Weapon {
    pub weapon_type: WeaponType,
    pub state: WeaponState,
    pub animation_frame: usize,
    pub animation_timer: Instant,
    pub frame_duration: Duration,
    pub ammo: u32,
    pub max_ammo: u32,
}

impl Weapon {
    pub fn new_pistol() -> Self {
        Self {
            weapon_type: WeaponType::Pistol,
            state: WeaponState::Idle,
            animation_frame: 0,
            animation_timer: Instant::now(),
            frame_duration: Duration::from_millis(100),
            ammo: 8,
            max_ammo: 8,
        }
    }

    pub fn fire(&mut self) -> bool {
        if self.ammo > 0 && self.state == WeaponState::Idle {
            self.ammo -= 1;
            self.state = WeaponState::Firing;
            self.animation_frame = 0;
            self.animation_timer = Instant::now();
            true // Successfully fired
        } else {
            false // Could not fire (no ammo or busy)
        }
    }

    pub fn reload(&mut self) {
        if self.ammo < self.max_ammo && self.state == WeaponState::Idle {
            self.state = WeaponState::Reloading;
            self.animation_frame = 0;
            self.animation_timer = Instant::now();
        }
    }

    pub fn update(&mut self) {
        if self.animation_timer.elapsed() >= self.frame_duration {
            match self.state {
                WeaponState::Firing => {
                    self.animation_frame += 1;
                    if self.animation_frame >= 1 { // Single firing frame
                        self.state = WeaponState::Recoil;
                        self.animation_frame = 0;
                    }
                }
                WeaponState::Recoil => {
                    self.animation_frame += 1;
                    if self.animation_frame >= 1 { // Single recoil frame
                        self.state = WeaponState::Idle;
                        self.animation_frame = 0;
                    }
                }
                WeaponState::Reloading => {
                    self.animation_frame += 1;
                    if self.animation_frame >= 3 { // 3 frame reload animation
                        self.ammo = self.max_ammo;
                        self.state = WeaponState::Idle;
                        self.animation_frame = 0;
                    }
                }
                WeaponState::Idle => {}
            }
            self.animation_timer = Instant::now();
        }
    }

    pub fn get_current_sprite(&self) -> WeaponSprite {
        match self.state {
            WeaponState::Idle => get_pistol_idle(),
            WeaponState::Firing => get_pistol_firing(),
            WeaponState::Recoil => get_pistol_recoil(),
            WeaponState::Reloading => get_pistol_reloading(),
        }
    }
}

#[derive(Clone)]
pub struct WeaponSprite {
    pub lines: Vec<String>,
    pub colors: Vec<Vec<Color>>,
    pub width: usize,
    pub height: usize,
}

impl WeaponSprite {
    pub fn new(ascii_lines: Vec<&str>, color_lines: Vec<Vec<Color>>) -> Self {
        let height = ascii_lines.len();
        let width = ascii_lines.iter().map(|line| line.len()).max().unwrap_or(0);
        
        Self {
            lines: ascii_lines.iter().map(|s| s.to_string()).collect(),
            colors: color_lines,
            width,
            height,
        }
    }
}

fn get_pistol_idle() -> WeaponSprite {
    WeaponSprite::new(
        vec![
            "        ████        ",
            "      ████████      ",
            "     ██████████     ",
            "    ████████████    ",
            "   ██████████████   ",
            "    ████████████    ",
            "     ██████████     ",
        ],
        vec![
            vec![Color::Rgb{r:120,g:120,b:120}; 20], // Light metal
            vec![Color::Rgb{r:100,g:100,b:100}; 20], // Metal
            vec![Color::Rgb{r:90,g:90,b:90}; 20],
            vec![Color::Rgb{r:80,g:80,b:80}; 20],
            vec![Color::Rgb{r:70,g:70,b:70}; 20],
            vec![Color::Rgb{r:60,g:60,b:60}; 20],
            vec![Color::Rgb{r:50,g:50,b:50}; 20],
        ]
    )
}

fn get_pistol_firing() -> WeaponSprite {
    WeaponSprite::new(
        vec![
            "    ████████████    ",
            "   ██████████████   ",
            "  ████████████████  ",
            " ██████████████████ ",
            "████████████████████",
            "    ████████████    ",
            "     ██████████     ",
        ],
        vec![
            vec![Color::Rgb{r:255,g:255,b:100}; 20], // Bright muzzle flash
            vec![Color::Rgb{r:255,g:200,b:50}; 20],  // Orange flash
            vec![Color::Rgb{r:255,g:150,b:0}; 20],   // Red flash
            vec![Color::Rgb{r:200,g:100,b:0}; 20],   // Deep red
            vec![Color::Rgb{r:150,g:150,b:150}; 20], // Gun metal
            vec![Color::Rgb{r:80,g:80,b:80}; 20],
            vec![Color::Rgb{r:60,g:60,b:60}; 20],
        ]
    )
}

fn get_pistol_recoil() -> WeaponSprite {
    WeaponSprite::new(
        vec![
            "                    ",
            "        ████        ",
            "      ████████      ",
            "     ██████████     ",
            "    ████████████    ",
            "   ██████████████   ",
            "     ██████████     ",
        ],
        vec![
            vec![Color::Rgb{r:0,g:0,b:0}; 20],       // Empty
            vec![Color::Rgb{r:100,g:100,b:100}; 20], // Metal
            vec![Color::Rgb{r:85,g:85,b:85}; 20],
            vec![Color::Rgb{r:70,g:70,b:70}; 20],
            vec![Color::Rgb{r:55,g:55,b:55}; 20],
            vec![Color::Rgb{r:40,g:40,b:40}; 20],
            vec![Color::Rgb{r:30,g:30,b:30}; 20],
        ]
    )
}

fn get_pistol_reloading() -> WeaponSprite {
    WeaponSprite::new(
        vec![
            "                    ",
            "                    ",
            "        ████        ",
            "      ████████      ",
            "     ██████████     ",
            "    ████████████    ",
            "     ██████████     ",
        ],
        vec![
            vec![Color::Rgb{r:0,g:0,b:0}; 20],       // Empty
            vec![Color::Rgb{r:0,g:0,b:0}; 20],       // Empty
            vec![Color::Rgb{r:150,g:150,b:50}; 20],  // Brass/reload color
            vec![Color::Rgb{r:120,g:120,b:40}; 20],  
            vec![Color::Rgb{r:90,g:90,b:90}; 20],
            vec![Color::Rgb{r:70,g:70,b:70}; 20],
            vec![Color::Rgb{r:50,g:50,b:50}; 20],
        ]
    )
}