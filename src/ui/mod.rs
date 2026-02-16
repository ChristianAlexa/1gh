pub(crate) mod colors;
mod gradient_bar;
mod modal;
mod sections;

use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(2),  // Title
        Constraint::Length(2),  // Timer/progress bar + hint
        Constraint::Length(6),  // Todos
        Constraint::Length(3),  // Action bar
        Constraint::Min(8),     // History
    ])
    .split(area);

    sections::draw_title(frame, chunks[0]);
    sections::draw_timer(frame, chunks[1], app);
    sections::draw_todos(frame, chunks[2], app);
    sections::draw_action_bar(frame, chunks[3], app);
    sections::draw_history(frame, chunks[4], app);

    if let Some(ref modal) = app.modal {
        modal::draw_modal(frame, area, modal);
    }
}
