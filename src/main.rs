use std::io::{stdout, Stdout, Write};
use std::time::Duration;
use crossterm;

fn main() {
    let mut stdout: Stdout = stdout();

    // Activer le mode brut du terminal
    terminal::enable_raw_mode();
    stdout.execute(EnterAlternateScreen);
    stdout.execute(Hide);

    loop {
        if poll(Duration::from_millis(500)) {
            if let Event::Key(event) = event::read() {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        stdout.flush();
    }

    // Désactivation propre du mode brut
    stdout.execute(Show);
    stdout.execute(LeaveAlternateScreen);
    terminal::disable_raw_mode();
}
