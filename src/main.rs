use std::io::{stdout, Write, Result};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode}; // To detect keyboard and mouse position
use enigo::{Enigo, MouseControllable}; // To move the mouse
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    cursor::MoveTo,
};

mod game;
use game::Game;

mod graphics;

mod level;

mod player;  // Importer le fichier player.rs
use player::Player;  // Importer la struct Player

mod mainGame;
use mainGame::MainGame;



fn main() -> Result<()>{
    let mut test = level::Level::new()?;

    let mut maze = Vec::new();
    let mut rng = rand::thread_rng();

    for y in 0..40 {
        if y == 0 || y == 39 {
            // Bord supérieur et inférieur rempli de #
            maze.push(vec!['#'; 160]);
        } else {
            // Bordures gauche et droite avec des espaces au milieu et des # aléatoires
            let mut line = vec!['#'];
            for x in 1..159 {
                // 5% chance to place a wall
                //if rng.gen_ratio(1, 20) {
                //    line.push('#');
                //} else {

                //}
                if (y == 20 && x<100 && x>50) || (x == 50 && y>20 && y<30){
                    line.push('#');
                }
                else{
                    line.push(' ');
                }
            }
            line.push('#');
            maze.push(line);
        }
    }

    let MAZE: &Vec<Vec<char>> = &maze;

    let mut mainGame = MainGame::new(MAZE);
    mainGame.init()?;

    let mut my_game = Game::new()?;
    my_game.launch()?;

    Ok(())
}

