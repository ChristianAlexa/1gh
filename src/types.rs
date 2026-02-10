pub const MAX_TIME: u64 = 3600;

pub fn format_time(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{mins}:{secs:02}")
}

#[derive(Debug, Clone, Default)]
pub struct Todo {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct CompletedNote {
    pub todos: Vec<Todo>,
    pub time_spent: u64,
    pub completion_number: u64,
}

#[derive(Debug, Clone)]
pub struct ActiveNote {
    pub todos: [Todo; 4],
    pub time_left: u64,
    pub is_running: bool,
    pub target_time: Option<i64>,
}

impl Default for ActiveNote {
    fn default() -> Self {
        Self {
            todos: std::array::from_fn(|_| Todo::default()),
            time_left: MAX_TIME,
            is_running: false,
            target_time: None,
        }
    }
}
