use std::io::{stdout, Stdout, Write, Result};
use std::time::Duration;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, poll, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

fn main() -> Result<()> {

    let write = io::Stdout().unwrap();

    !queue(
        write,
        EnterAlternateScreen,
        Hide
    )?;

    for i in 1..10 {
        !queue(write, )
    }


    Ok(())
}
