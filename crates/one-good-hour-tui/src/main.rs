mod event;
mod ui;

use std::io;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use one_good_hour_core::app::App;

fn main() -> Result<()> {
    let mut app = App::new();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app loop
    let result = run(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

/// Height of fixed UI sections (title + timer + todos + action bar)
const COMPACT_HEIGHT: u16 = 13;

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let mut was_showing_history = app.show_history;
    let mut saved_height: Option<u16> = None;

    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        event::handle_events(app)?;

        // Resize terminal window when history visibility changes
        if app.show_history != was_showing_history {
            let (cols, rows) = terminal::size()?;
            if app.show_history {
                if let Some(h) = saved_height.take() {
                    resize_window(h, cols);
                }
            } else {
                saved_height = Some(rows);
                resize_window(COMPACT_HEIGHT, cols);
            }
            was_showing_history = app.show_history;
        }

        if app.sound_pending {
            print!("\x07");
            app.sound_pending = false;
        }

        if app.should_quit {
            // Restore terminal size if history is hidden on quit
            if let Some(h) = saved_height {
                let (cols, _) = terminal::size().unwrap_or((80, 24));
                resize_window(h, cols);
            }
            return Ok(());
        }
    }
}

/// Resize the terminal window using xterm escape sequence.
fn resize_window(rows: u16, cols: u16) {
    print!("\x1b[8;{rows};{cols}t");
}
