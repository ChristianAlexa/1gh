use serde::{Deserialize, Serialize};

use crate::types::*;

const MAX_TODO_LEN: usize = 50;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputMode {
    Normal,
    Editing(usize),
    Modal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModalKind {
    CompleteSession,
    ClearNotes,
    NewSession,
    Help,
}

pub struct App {
    pub active_note: ActiveNote,
    pub completed_notes: Vec<CompletedNote>,
    pub input_mode: InputMode,
    pub selected_todo: usize,
    pub history_index: Option<usize>,
    pub modal: Option<ModalKind>,
    pub should_quit: bool,
    pub status_message: Option<String>,
    pub sound_pending: bool,
    pub show_history: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_note: ActiveNote::default(),
            completed_notes: Vec::new(),
            input_mode: InputMode::Normal,
            selected_todo: 0,
            history_index: None,
            modal: None,
            should_quit: false,
            status_message: None,
            sound_pending: false,
            show_history: true,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    // Timer controls

    pub fn toggle_timer(&mut self) {
        if self.active_note.is_running {
            // Pause
            self.active_note.is_running = false;
            self.active_note.target_time = None;
        } else {
            // Play — guard: must have time left
            if self.active_note.time_left == 0 {
                return;
            }
            self.active_note.is_running = true;
            let now_ms = chrono::Utc::now().timestamp_millis();
            self.active_note.target_time =
                Some(now_ms + (self.active_note.time_left as i64 * 1000));
        }
    }

    pub fn reset_timer(&mut self) {
        self.active_note.time_left = MAX_TIME;
        self.active_note.is_running = false;
        self.active_note.target_time = None;
    }

    pub fn tick(&mut self) {
        if !self.active_note.is_running {
            return;
        }
        if let Some(target) = self.active_note.target_time {
            let now_ms = chrono::Utc::now().timestamp_millis();
            let remaining = ((target - now_ms) as f64 / 1000.0).ceil() as i64;
            if remaining > 0 {
                self.active_note.time_left = remaining as u64;
            } else {
                self.active_note.time_left = 0;
                self.active_note.is_running = false;
                self.active_note.target_time = None;
                self.play_sound();
            }
        }
    }

    // Todo operations

    pub fn move_selection_down(&mut self) {
        self.selected_todo = (self.selected_todo + 1).min(self.active_note.todos.len() - 1);
    }

    pub fn move_selection_up(&mut self) {
        self.selected_todo = self.selected_todo.saturating_sub(1);
    }

    pub fn start_editing(&mut self) {
        self.input_mode = InputMode::Editing(self.selected_todo);
    }

    pub fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn edit_char(&mut self, c: char, index: usize) {
        if self.active_note.todos[index].text.len() < MAX_TODO_LEN {
            self.active_note.todos[index].text.push(c);
        }
    }

    pub fn edit_backspace(&mut self, index: usize) {
        self.active_note.todos[index].text.pop();
    }

    pub fn toggle_todo(&mut self) {
        let idx = self.selected_todo;
        // Only toggle if todo has text
        if !self.active_note.todos[idx].text.is_empty() {
            self.active_note.todos[idx].completed = !self.active_note.todos[idx].completed;
            if self.active_note.todos[idx].completed {
                self.play_sound();
            }
        }
    }

    pub fn remove_todo(&mut self) {
        let idx = self.selected_todo;
        self.active_note.todos[idx].text.clear();
        self.active_note.todos[idx].completed = false;
    }

    // Session completion

    pub fn show_complete_session_modal(&mut self) {
        // Guard: at least one todo must have text
        let has_content = self.active_note.todos.iter().any(|t| !t.text.is_empty());
        if !has_content {
            return;
        }
        self.modal = Some(ModalKind::CompleteSession);
        self.input_mode = InputMode::Modal;
    }

    fn complete_session(&mut self) {
        let time_spent = MAX_TIME - self.active_note.time_left;
        let completion_number = self.completed_notes.len() as u64 + 1;

        let note = CompletedNote {
            todos: self.active_note.todos.to_vec(),
            time_spent,
            completion_number,
        };

        self.completed_notes.push(note);
        self.history_index = Some(self.completed_notes.len() - 1);

        // Reset active note
        self.active_note = ActiveNote::default();
        self.selected_todo = 0;
    }

    // History navigation

    pub fn next_history(&mut self) {
        if self.completed_notes.is_empty() {
            return;
        }
        match self.history_index {
            Some(i) if i + 1 < self.completed_notes.len() => {
                self.history_index = Some(i + 1);
            }
            None if !self.completed_notes.is_empty() => {
                self.history_index = Some(0);
            }
            _ => {}
        }
    }

    pub fn prev_history(&mut self) {
        if self.completed_notes.is_empty() {
            return;
        }
        match self.history_index {
            Some(i) if i > 0 => {
                self.history_index = Some(i - 1);
            }
            None if !self.completed_notes.is_empty() => {
                self.history_index = Some(self.completed_notes.len() - 1);
            }
            _ => {}
        }
    }

    // Modals

    pub fn show_new_session_modal(&mut self) {
        self.modal = Some(ModalKind::NewSession);
        self.input_mode = InputMode::Modal;
    }

    pub fn show_clear_notes_modal(&mut self) {
        if self.completed_notes.is_empty() {
            return;
        }
        self.modal = Some(ModalKind::ClearNotes);
        self.input_mode = InputMode::Modal;
    }

    pub fn show_help(&mut self) {
        self.modal = Some(ModalKind::Help);
        self.input_mode = InputMode::Modal;
    }

    pub fn confirm_modal(&mut self) {
        if let Some(modal) = self.modal.take() {
            match modal {
                ModalKind::CompleteSession => {
                    self.complete_session();
                }
                ModalKind::ClearNotes => {
                    self.completed_notes.clear();
                    self.history_index = None;
                }
                ModalKind::NewSession => {
                    self.active_note = ActiveNote::default();
                    self.completed_notes.clear();
                    self.selected_todo = 0;
                    self.history_index = None;
                }
                ModalKind::Help => {}
            }
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn dismiss_modal(&mut self) {
        self.modal = None;
        self.input_mode = InputMode::Normal;
    }

    // History visibility

    pub fn toggle_history(&mut self) {
        self.show_history = !self.show_history;
    }

    // Sound

    fn play_sound(&mut self) {
        self.sound_pending = true;
    }

    // Clipboard

    pub fn copy_markdown(&mut self) {
        if self.completed_notes.is_empty() {
            return;
        }
        let md = self.build_markdown();
        match arboard::Clipboard::new().and_then(|mut cb| cb.set_text(&md)) {
            Ok(_) => {
                self.status_message = Some("Copied to clipboard!".to_string());
            }
            Err(e) => {
                self.status_message = Some(format!("Clipboard error: {e}"));
            }
        }
    }

    fn build_markdown(&self) -> String {
        let mut md = String::from("# One Good Hour\n\n");
        for note in &self.completed_notes {
            md.push_str(&format!("## Session {}\n", note.completion_number));
            md.push_str(&format!("Time spent: {}\n\n", format_time(note.time_spent)));
            for todo in &note.todos {
                if !todo.text.is_empty() {
                    let check = if todo.completed { "x" } else { " " };
                    md.push_str(&format!("- [{}] {}\n", check, todo.text));
                }
            }
            md.push('\n');
        }
        md
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn app_with_todo(text: &str) -> App {
        let mut app = App::new();
        app.active_note.todos[0].text = text.to_string();
        app
    }

    // -- Selection movement --

    #[test]
    fn move_selection_down_increments() {
        let mut app = App::new();
        assert_eq!(app.selected_todo, 0);
        app.move_selection_down();
        assert_eq!(app.selected_todo, 1);
        app.move_selection_down();
        assert_eq!(app.selected_todo, 2);
        app.move_selection_down();
        assert_eq!(app.selected_todo, 3);
    }

    #[test]
    fn move_selection_down_clamps_at_max() {
        let mut app = App::new();
        app.selected_todo = 3;
        app.move_selection_down();
        assert_eq!(app.selected_todo, 3);
    }

    #[test]
    fn move_selection_up_decrements() {
        let mut app = App::new();
        app.selected_todo = 3;
        app.move_selection_up();
        assert_eq!(app.selected_todo, 2);
        app.move_selection_up();
        assert_eq!(app.selected_todo, 1);
        app.move_selection_up();
        assert_eq!(app.selected_todo, 0);
    }

    #[test]
    fn move_selection_up_clamps_at_zero() {
        let mut app = App::new();
        app.move_selection_up();
        assert_eq!(app.selected_todo, 0);
    }

    // -- Editing --

    #[test]
    fn start_and_stop_editing() {
        let mut app = App::new();
        app.selected_todo = 2;
        app.start_editing();
        assert_eq!(app.input_mode, InputMode::Editing(2));
        app.stop_editing();
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn edit_char_appends() {
        let mut app = App::new();
        app.edit_char('h', 0);
        app.edit_char('i', 0);
        assert_eq!(app.active_note.todos[0].text, "hi");
    }

    #[test]
    fn edit_char_respects_max_len() {
        let mut app = App::new();
        for _ in 0..MAX_TODO_LEN {
            app.edit_char('a', 0);
        }
        assert_eq!(app.active_note.todos[0].text.len(), MAX_TODO_LEN);
        app.edit_char('b', 0);
        assert_eq!(app.active_note.todos[0].text.len(), MAX_TODO_LEN);
    }

    #[test]
    fn edit_backspace_removes_last() {
        let mut app = app_with_todo("hello");
        app.edit_backspace(0);
        assert_eq!(app.active_note.todos[0].text, "hell");
    }

    #[test]
    fn edit_backspace_on_empty_is_safe() {
        let mut app = App::new();
        app.edit_backspace(0); // should not panic
        assert_eq!(app.active_note.todos[0].text, "");
    }

    // -- Toggle todo --

    #[test]
    fn toggle_todo_with_text() {
        let mut app = app_with_todo("task");
        assert!(!app.active_note.todos[0].completed);
        app.toggle_todo();
        assert!(app.active_note.todos[0].completed);
        app.toggle_todo();
        assert!(!app.active_note.todos[0].completed);
    }

    #[test]
    fn toggle_todo_empty_is_noop() {
        let mut app = App::new();
        app.toggle_todo();
        assert!(!app.active_note.todos[0].completed);
    }

    // -- Remove todo --

    #[test]
    fn remove_todo_clears_text_and_completed() {
        let mut app = app_with_todo("task");
        app.active_note.todos[0].completed = true;
        app.remove_todo();
        assert_eq!(app.active_note.todos[0].text, "");
        assert!(!app.active_note.todos[0].completed);
    }

    // -- Timer --

    #[test]
    fn toggle_timer_starts_and_pauses() {
        let mut app = App::new();
        assert!(!app.active_note.is_running);
        app.toggle_timer();
        assert!(app.active_note.is_running);
        assert!(app.active_note.target_time.is_some());
        app.toggle_timer();
        assert!(!app.active_note.is_running);
        assert!(app.active_note.target_time.is_none());
    }

    #[test]
    fn toggle_timer_blocked_at_zero() {
        let mut app = App::new();
        app.active_note.time_left = 0;
        app.toggle_timer();
        assert!(!app.active_note.is_running);
    }

    #[test]
    fn reset_timer_restores_defaults() {
        let mut app = App::new();
        app.active_note.time_left = 100;
        app.active_note.is_running = true;
        app.active_note.target_time = Some(999);
        app.reset_timer();
        assert_eq!(app.active_note.time_left, MAX_TIME);
        assert!(!app.active_note.is_running);
        assert!(app.active_note.target_time.is_none());
    }

    // -- Session completion --

    #[test]
    fn show_complete_session_modal_requires_content() {
        let mut app = App::new();
        app.show_complete_session_modal();
        assert!(app.modal.is_none()); // all todos empty, no modal

        app.active_note.todos[1].text = "something".to_string();
        app.show_complete_session_modal();
        assert!(matches!(app.modal, Some(ModalKind::CompleteSession)));
        assert_eq!(app.input_mode, InputMode::Modal);
    }

    #[test]
    fn complete_session_via_confirm() {
        let mut app = app_with_todo("task 1");
        app.active_note.todos[1].text = "task 2".to_string();
        app.active_note.todos[1].completed = true;
        app.active_note.time_left = 1800; // used half the time

        app.show_complete_session_modal();
        app.confirm_modal();

        assert_eq!(app.completed_notes.len(), 1);
        let note = &app.completed_notes[0];
        assert_eq!(note.completion_number, 1);
        assert_eq!(note.time_spent, 1800);
        assert_eq!(note.todos[0].text, "task 1");
        assert!(note.todos[1].completed);

        // Active note should be reset
        assert_eq!(app.active_note.time_left, MAX_TIME);
        assert!(app.active_note.todos[0].text.is_empty());
        assert_eq!(app.selected_todo, 0);
        assert_eq!(app.history_index, Some(0));
    }

    #[test]
    fn dismiss_modal_clears_state() {
        let mut app = app_with_todo("task");
        app.show_complete_session_modal();
        app.dismiss_modal();
        assert!(app.modal.is_none());
        assert_eq!(app.input_mode, InputMode::Normal);
        assert!(app.completed_notes.is_empty()); // session NOT completed
    }

    // -- History navigation --

    fn app_with_history(count: usize) -> App {
        let mut app = App::new();
        for i in 0..count {
            app.active_note.todos[0].text = format!("session {}", i + 1);
            app.show_complete_session_modal();
            app.confirm_modal();
        }
        app
    }

    #[test]
    fn history_navigation_empty() {
        let mut app = App::new();
        app.next_history();
        assert_eq!(app.history_index, None);
        app.prev_history();
        assert_eq!(app.history_index, None);
    }

    #[test]
    fn next_history_from_none_starts_at_zero() {
        let mut app = app_with_history(3);
        app.history_index = None;
        app.next_history();
        assert_eq!(app.history_index, Some(0));
    }

    #[test]
    fn prev_history_from_none_starts_at_last() {
        let mut app = app_with_history(3);
        app.history_index = None;
        app.prev_history();
        assert_eq!(app.history_index, Some(2));
    }

    #[test]
    fn next_history_clamps_at_end() {
        let mut app = app_with_history(2);
        app.history_index = Some(1);
        app.next_history();
        assert_eq!(app.history_index, Some(1)); // stays at last
    }

    #[test]
    fn prev_history_clamps_at_start() {
        let mut app = app_with_history(2);
        app.history_index = Some(0);
        app.prev_history();
        assert_eq!(app.history_index, Some(0)); // stays at first
    }

    #[test]
    fn history_navigation_walks_through() {
        let mut app = app_with_history(3);
        app.history_index = Some(0);
        app.next_history();
        assert_eq!(app.history_index, Some(1));
        app.next_history();
        assert_eq!(app.history_index, Some(2));
        app.prev_history();
        assert_eq!(app.history_index, Some(1));
    }

    // -- Clear notes --

    #[test]
    fn clear_notes_modal_requires_history() {
        let mut app = App::new();
        app.show_clear_notes_modal();
        assert!(app.modal.is_none());
    }

    #[test]
    fn clear_notes_via_confirm() {
        let mut app = app_with_history(2);
        assert_eq!(app.completed_notes.len(), 2);
        app.show_clear_notes_modal();
        app.confirm_modal();
        assert!(app.completed_notes.is_empty());
        assert_eq!(app.history_index, None);
    }

    // -- Build markdown --

    #[test]
    fn build_markdown_format() {
        let mut app = app_with_todo("write tests");
        app.active_note.todos[0].completed = true;
        app.active_note.todos[1].text = "review PR".to_string();
        app.active_note.time_left = 2400; // 20 min spent

        app.show_complete_session_modal();
        app.confirm_modal();

        let md = app.build_markdown();
        assert!(md.contains("# One Good Hour"));
        assert!(md.contains("## Session 1"));
        assert!(md.contains("Time spent: 20:00"));
        assert!(md.contains("- [x] write tests"));
        assert!(md.contains("- [ ] review PR"));
    }

    // -- Toggle history --

    #[test]
    fn show_history_defaults_to_true() {
        let app = App::new();
        assert!(app.show_history);
    }

    #[test]
    fn toggle_history_flips() {
        let mut app = App::new();
        assert!(app.show_history);
        app.toggle_history();
        assert!(!app.show_history);
        app.toggle_history();
        assert!(app.show_history);
    }
}
