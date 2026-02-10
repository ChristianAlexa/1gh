use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::ModalKind;

pub(super) fn draw_modal(frame: &mut Frame, area: Rect, modal: &ModalKind) {
    if matches!(modal, ModalKind::Help) {
        draw_help_modal(frame, area);
        return;
    }

    let (title, body) = match modal {
        ModalKind::CompleteSession => (
            "Complete Session",
            "Complete this session and save to history?\n\n[y] Yes  [n] No",
        ),
        ModalKind::ClearNotes => (
            "Clear History",
            "Clear all completed sessions?\n\n[y] Yes  [n] No",
        ),
        ModalKind::Help => unreachable!(),
    };

    let modal_area = centered_rect_fixed(40, 7, area);
    frame.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let para = Paragraph::new(body)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
        .block(block);

    frame.render_widget(para, modal_area);
}

fn draw_help_modal(frame: &mut Frame, area: Rect) {
    let modal_area = centered_rect(60, 70, area);
    frame.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(" Shortcuts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let key = Style::default().fg(Color::Yellow);

    let shortcuts: &[(&str, &str, &str, &str)] = &[
        ("Space", "Play/Pause timer", "j/↓", "Move down"),
        ("r", "Reset timer", "k/↑", "Move up"),
        ("Enter", "Edit task", "x", "Check off task"),
        ("d", "Clear task", "c", "Complete session"),
        ("h/←", "Prev history", "l/→", "Next history"),
        ("y", "Copy markdown", "D", "Clear history"),
        ("q", "Quit", "?", "This help"),
    ];

    let mut lines: Vec<Line> = vec![Line::from("")];
    for &(k1, d1, k2, d2) in shortcuts {
        lines.push(Line::from(vec![
            Span::styled(format!("{k1:>7}"), key),
            Span::raw(format!("  {d1:<18}")),
            Span::styled(format!("{k2:>5}"), key),
            Span::raw(format!("  {d2}")),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "            [Esc] Close",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(lines).block(block);

    frame.render_widget(para, modal_area);
}

fn centered_rect_fixed(percent_x: u16, height: u16, area: Rect) -> Rect {
    let vert = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(height),
        Constraint::Min(0),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vert[1])[1]
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vert = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vert[1])[1]
}
