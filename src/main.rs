extern crate termion;

use std::{thread, time};
use termion::{clear, color, cursor, terminal_size};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};


fn fragment(uv: [f32; 2], time: f32) -> [f32; 3] {
    [uv[0], uv[1], time % 1.]
}

fn main() {
    println!("{}", clear::All);
    let mut timer: f32 = 0.;

    // let mut stdout = stdout().into_raw_mode().unwrap();
    // write!(stdout, "Hey there.").unwrap();

    loop {
        let mut screen_str = String::new();

        let term_size = match terminal_size() {
            Ok((width, height)) => (width as f32, height as f32),
            Err(_) => {
                eprintln!("Failed to get terminal size");
                return;
            }
        };

        for i in 0..term_size.0 as u16 {
            for j in 0..term_size.1 as u16 {
                let uv = [i as f32 / term_size.0, j as f32 / term_size.1];
                let color = fragment(uv, timer);

                screen_str.push_str(&format!(
                    "{}{} ",
                    cursor::Goto(i + 1, j + 1),
                    color::Bg(color::Rgb(
                        (color[0] * 255.0) as u8,
                        (color[1] * 255.0) as u8,
                        (color[2] * 255.0) as u8
                    ))
                ));
            }
        }
    print!("{}", screen_str);

    thread::sleep(time::Duration::from_millis(100));
    timer += 0.1;
    }

}
