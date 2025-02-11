use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    queue, execute,
    cursor::{Hide, Show, MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Print, SetBackgroundColor, Color},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<()> {

    let mut write = std::io::stdout();

    terminal::enable_raw_mode()?;
    execute!(write,
        EnterAlternateScreen,
        Hide
    )?;

    let time_launch = Instant::now();
    let mut time_of_last_loop = time_launch;

    loop {
        let term_size = terminal::size()?;

        let time_delta = time_of_last_loop.elapsed();
        time_of_last_loop += time_delta;

        for j in 0..term_size.1 {
            for i in 0..term_size.0 {
                let uv = {(
                    i as f32 / term_size.0 as f32,
                    j as f32 / term_size.1 as f32
                )};

                queue!(write,
                    MoveTo(i, j),
                    SetBackgroundColor(Color::Rgb{
                        r: (uv.0 * 255.0) as u8,
                        g: (uv.1 * 255.0) as u8,
                        b: ( ((time_launch.elapsed().as_millis() as f32 / 1000_f32).sin() + 1_f32) * 128_f32 ) as u8
                    }),
                    Print(' ')
                )?;
            }
        }
        queue!(write,
            MoveTo(0, 0),
            Print(time_launch.elapsed().as_millis())
        )?;

        write.flush()?;

        if event::poll(Duration::from_millis(50))?{
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char('c') &&
                   key_event.modifiers.contains(KeyModifiers::CONTROL)
                { break; }
            }
        }
    }

    execute!(write,
        LeaveAlternateScreen,
        Show
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
