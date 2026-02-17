use serde::{Deserialize, Serialize};

pub const MAX_TIME: u64 = 3600;

pub fn format_time(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{mins}:{secs:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_time_zero() {
        assert_eq!(format_time(0), "0:00");
    }

    #[test]
    fn format_time_seconds_only() {
        assert_eq!(format_time(59), "0:59");
    }

    #[test]
    fn format_time_exact_minute() {
        assert_eq!(format_time(60), "1:00");
    }

    #[test]
    fn format_time_mixed() {
        assert_eq!(format_time(90), "1:30");
        assert_eq!(format_time(61), "1:01");
    }

    #[test]
    fn format_time_max() {
        assert_eq!(format_time(MAX_TIME), "60:00");
    }

    #[test]
    fn format_time_pads_seconds() {
        assert_eq!(format_time(5), "0:05");
        assert_eq!(format_time(65), "1:05");
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Todo {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedNote {
    pub todos: Vec<Todo>,
    pub time_spent: u64,
    pub completion_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
