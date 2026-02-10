use crate::types::*;

const MAX_TODO_LEN: usize = 50;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing(usize),
    Modal,
}

#[derive(Debug, Clone)]
pub enum ModalKind {
    CompleteSession,
    ClearNotes,
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
}

impl App {
    pub fn new() -> Self {
        Self {
            active_note: ActiveNote::default(),
            completed_notes: Vec::new(),
            input_mode: InputMode::Normal,
            selected_todo: 0,
            history_index: None,
            modal: None,
            should_quit: false,
            status_message: None,
        }
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
                ModalKind::Help => {}
            }
        }
        self.input_mode = InputMode::Normal;
    }

    pub fn dismiss_modal(&mut self) {
        self.modal = None;
        self.input_mode = InputMode::Normal;
    }

    // Sound

    fn play_sound(&self) {
        print!("\x07");
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
