use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, InputMode};

const TICK_RATE: Duration = Duration::from_millis(250);

pub fn handle_events(app: &mut App) -> anyhow::Result<()> {
    if event::poll(TICK_RATE)? {
        if let Event::Key(key) = event::read()? {
            // Ignore key release events (crossterm on Windows sends both press and release)
            if key.kind != event::KeyEventKind::Press {
                return Ok(());
            }
            match &app.input_mode {
                InputMode::Normal => handle_normal(app, key),
                InputMode::Editing(idx) => handle_editing(app, key, *idx),
                InputMode::Modal => handle_modal(app, key),
            }
        }
    }
    // Always tick the timer
    app.tick();
    Ok(())
}

fn handle_normal(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true
        }

        // Timer
        KeyCode::Char(' ') => app.toggle_timer(),
        KeyCode::Char('r') => app.reset_timer(),

        // Todo navigation
        KeyCode::Char('j') | KeyCode::Down => app.move_selection_down(),
        KeyCode::Char('k') | KeyCode::Up => app.move_selection_up(),

        // Todo actions
        KeyCode::Enter => app.start_editing(),
        KeyCode::Char('x') => app.toggle_todo(),
        KeyCode::Char('d') => app.remove_todo(),

        // Session
        KeyCode::Char('c') => app.show_complete_session_modal(),
        // History
        KeyCode::Char('h') | KeyCode::Left => app.prev_history(),
        KeyCode::Char('l') | KeyCode::Right => app.next_history(),

        // Clipboard
        KeyCode::Char('y') => app.copy_markdown(),
        KeyCode::Char('D') => app.show_clear_notes_modal(),

        // Help
        KeyCode::Char('?') => app.show_help(),

        _ => {}
    }
    // Clear status message on any keypress
    app.status_message = None;
}

fn handle_editing(app: &mut App, key: KeyEvent, index: usize) {
    match key.code {
        KeyCode::Enter | KeyCode::Esc => app.stop_editing(),
        KeyCode::Backspace => app.edit_backspace(index),
        KeyCode::Char(c) => app.edit_char(c, index),
        _ => {}
    }
}

fn handle_modal(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Enter => app.confirm_modal(),
        KeyCode::Char('n') | KeyCode::Esc => app.dismiss_modal(),
        _ => {}
    }
}
