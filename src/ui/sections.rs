use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, InputMode};
use crate::types::{format_time, Todo, MAX_TIME};

use super::gradient_bar::GradientBar;

const COLOR_SELECTION_BG: Color = Color::Rgb(30, 30, 50);

pub(super) fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("ONE GOOD HOUR")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, area);
}

pub(super) fn draw_timer(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(1), // Progress bar with timer
        Constraint::Length(1), // Controls hint
    ])
    .split(area);

    // Progress bar with embedded timer
    let progress = 1.0 - (app.active_note.time_left as f64 / MAX_TIME as f64);
    let timer_text = format_time(app.active_note.time_left);
    let bar = GradientBar {
        ratio: progress.clamp(0.0, 1.0),
        label: format!("{timer_text} · {}%", (progress * 100.0) as u16),
    };
    frame.render_widget(bar, chunks[0]);

    // Controls hint
    let status = if app.active_note.is_running {
        "▶ Running"
    } else if app.active_note.time_left == 0 {
        "✓ Done"
    } else {
        "⏸ Paused"
    };
    let hint = Paragraph::new(format!(
        "{status}  [Space] Play/Pause  [r] Reset"
    ))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(hint, chunks[1]);
}

pub(super) fn draw_todos(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Tasks ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(inner);

    for (i, todo) in app.active_note.todos.iter().enumerate() {
        let is_selected = i == app.selected_todo;
        let is_editing = app.input_mode == InputMode::Editing(i);

        let checkbox = if todo.completed { "[x]" } else { "[ ]" };
        let text = if todo.text.is_empty() && !is_editing {
            "(empty)".to_string()
        } else if is_editing {
            format!("{}▎", todo.text)
        } else {
            todo.text.clone()
        };

        let line = Line::from(vec![
            Span::styled(
                if is_selected { "▸ " } else { "  " },
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                format!("{checkbox} "),
                Style::default().fg(if todo.completed {
                    Color::Green
                } else {
                    Color::White
                }),
            ),
            Span::styled(
                format!("{}. ", i + 1),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(text, todo_text_style(todo, is_editing)),
        ]);

        let style = if is_selected {
            Style::default().bg(COLOR_SELECTION_BG)
        } else {
            Style::default()
        };

        let para = Paragraph::new(line).style(style);
        frame.render_widget(para, rows[i]);
    }
}

fn todo_text_style(todo: &Todo, is_editing: bool) -> Style {
    let fg = if is_editing {
        Color::Yellow
    } else if todo.text.is_empty() {
        Color::DarkGray
    } else if todo.completed {
        Color::Green
    } else {
        Color::White
    };
    let modifier = if todo.completed {
        Modifier::CROSSED_OUT
    } else {
        Modifier::empty()
    };
    Style::default().fg(fg).add_modifier(modifier)
}

pub(super) fn draw_action_bar(frame: &mut Frame, area: Rect, app: &App) {
    let status = if let Some(ref msg) = app.status_message {
        Line::from(Span::styled(msg.as_str(), Style::default().fg(Color::Green)))
    } else {
        Line::from(vec![
            Span::styled("[x]", Style::default().fg(Color::Yellow)),
            Span::raw(" Check  "),
            Span::styled("[c]", Style::default().fg(Color::Yellow)),
            Span::raw(" Complete  "),
            Span::styled("[?]", Style::default().fg(Color::Yellow)),
            Span::raw(" Help"),
        ])
    };

    let bar = Paragraph::new(status)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP | Borders::BOTTOM)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(bar, area);
}

pub(super) fn draw_history(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" History ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.completed_notes.is_empty() {
        let empty = Paragraph::new("No completed sessions yet. Complete a session with [c].")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(empty, inner);
        return;
    }

    let idx = app.history_index.unwrap_or(0);
    let note = &app.completed_notes[idx];
    let total = app.completed_notes.len();

    let chunks = Layout::vertical([
        Constraint::Length(1), // Header
        Constraint::Min(1),   // Todos
        Constraint::Length(1), // Footer
    ])
    .split(inner);

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("[←/h] ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("Session {} of {}", idx + 1, total),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" [→/l]", Style::default().fg(Color::DarkGray)),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Todos in the completed note
    let mut lines = Vec::new();
    for todo in &note.todos {
        if !todo.text.is_empty() {
            let check = if todo.completed { "[x]" } else { "[ ]" };
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {check} "),
                    Style::default().fg(if todo.completed {
                        Color::Green
                    } else {
                        Color::Red
                    }),
                ),
                Span::styled(
                    todo.text.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(if todo.completed {
                            Modifier::CROSSED_OUT
                        } else {
                            Modifier::empty()
                        }),
                ),
            ]));
        }
    }
    let todos_para = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(todos_para, chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("Time: {}", format_time(note.time_spent)),
            Style::default().fg(Color::DarkGray),
        ),
        Span::raw("  "),
        Span::styled("[y]", Style::default().fg(Color::Yellow)),
        Span::raw(" Copy  "),
        Span::styled("[D]", Style::default().fg(Color::Red)),
        Span::raw(" Clear"),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}
