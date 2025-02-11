use std::io::{Write, Result};
use std::time::Duration;
use crossterm::{
    queue, execute,
    cursor::{Hide, Show, MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<()> {

    let mut write = std::io::stdout();

    terminal::enable_raw_mode()?;
    execute!(write,
        EnterAlternateScreen,
        Hide
    )?;

    loop {

        for j in 0..terminal::size().unwrap().1 {
            for i in 0..terminal::size().unwrap().0 {
                queue!(write,
                    MoveTo(i, j),
                    SetBackgroundColor(Color(Rgb())),
                    Print('#')
                )?;
            }
        }

        write.flush()?;

        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('c') &&
                   key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }
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
