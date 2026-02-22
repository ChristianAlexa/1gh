use std::sync::Mutex;

use one_good_hour_core::app::{App, InputMode, ModalKind};
use one_good_hour_core::types::{format_time, MAX_TIME};
use serde::Serialize;
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::{AppHandle, LogicalSize, Manager, State, WebviewWindow};

pub struct AppState(pub Mutex<App>);
pub struct SavedWindowHeight(pub Mutex<Option<f64>>);
pub struct TrayHolder(pub Mutex<Option<TrayIcon>>);

#[derive(Debug, Clone, Serialize)]
pub struct TodoSnapshot {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteSnapshot {
    pub todos: Vec<TodoSnapshot>,
    pub time_spent: String,
    pub completion_number: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppSnapshot {
    pub time_left: u64,
    pub is_running: bool,
    pub progress: f64,
    pub timer_display: String,
    pub todos: Vec<TodoSnapshot>,
    pub selected_todo: usize,
    pub input_mode: String,
    pub editing_index: Option<usize>,
    pub modal: Option<String>,
    pub completed_notes: Vec<NoteSnapshot>,
    pub history_index: Option<usize>,
    pub history_total: usize,
    pub status_message: Option<String>,
    pub sound_pending: bool,
    pub show_history: bool,
}

fn snapshot(app: &mut App) -> AppSnapshot {
    let time_left = app.active_note.time_left;
    let progress = (1.0 - time_left as f64 / MAX_TIME as f64).clamp(0.0, 1.0);
    let timer_display = format_time(time_left);

    let todos: Vec<TodoSnapshot> = app
        .active_note
        .todos
        .iter()
        .map(|t| TodoSnapshot {
            text: t.text.clone(),
            completed: t.completed,
        })
        .collect();

    let (input_mode, editing_index) = match &app.input_mode {
        InputMode::Normal => ("normal".to_string(), None),
        InputMode::Editing(i) => (format!("editing:{i}"), Some(*i)),
        InputMode::Modal => ("modal".to_string(), None),
    };

    let modal = app.modal.as_ref().map(|m| match m {
        ModalKind::CompleteSession => "complete_session".to_string(),
        ModalKind::ClearNotes => "clear_notes".to_string(),
        ModalKind::NewSession => "new_session".to_string(),
        ModalKind::Help => "help".to_string(),
    });

    let completed_notes: Vec<NoteSnapshot> = app
        .completed_notes
        .iter()
        .map(|n| NoteSnapshot {
            todos: n
                .todos
                .iter()
                .map(|t| TodoSnapshot {
                    text: t.text.clone(),
                    completed: t.completed,
                })
                .collect(),
            time_spent: format_time(n.time_spent),
            completion_number: n.completion_number,
        })
        .collect();

    let sound_pending = app.sound_pending;
    if app.sound_pending {
        app.sound_pending = false;
    }

    let status_message = app.status_message.take();

    AppSnapshot {
        time_left,
        is_running: app.active_note.is_running,
        progress,
        timer_display,
        todos,
        selected_todo: app.selected_todo,
        input_mode,
        editing_index,
        modal,
        completed_notes,
        history_index: app.history_index,
        history_total: app.completed_notes.len(),
        status_message,
        sound_pending,
        show_history: app.show_history,
    }
}

#[tauri::command]
fn get_state(state: State<'_, AppState>) -> AppSnapshot {
    let mut app = state.0.lock().unwrap();
    snapshot(&mut app)
}

#[tauri::command]
fn tick(state: State<'_, AppState>, app_handle: AppHandle) -> AppSnapshot {
    let mut app = state.0.lock().unwrap();
    app.tick();
    let snap = snapshot(&mut app);
    if let Some(tray) = app_handle.tray_by_id("main-tray") {
        let prefix = if snap.is_running { "\u{25cf}" } else { "\u{25cb}" };
        let mins = (snap.time_left + 59) / 60; // round up
        let _ = tray.set_title(Some(format!("{} {}m", prefix, mins)));
    }
    snap
}

#[tauri::command]
fn action(
    name: String,
    payload: Option<String>,
    state: State<'_, AppState>,
    saved_height: State<'_, SavedWindowHeight>,
    webview: WebviewWindow,
) -> AppSnapshot {
    let mut app = state.0.lock().unwrap();
    match name.as_str() {
        "toggle_timer" => app.toggle_timer(),
        "reset_timer" => app.reset_timer(),
        "move_up" => app.move_selection_up(),
        "move_down" => app.move_selection_down(),
        "start_editing" => app.start_editing(),
        "stop_editing" => app.stop_editing(),
        "toggle_todo" => app.toggle_todo(),
        "remove_todo" => app.remove_todo(),
        "complete_session" => app.show_complete_session_modal(),
        "confirm_modal" => app.confirm_modal(),
        "dismiss_modal" => app.dismiss_modal(),
        "next_history" => app.next_history(),
        "prev_history" => app.prev_history(),
        "copy_markdown" => app.copy_markdown(),
        "clear_notes" => app.show_clear_notes_modal(),
        "new_session" => app.show_new_session_modal(),
        "show_help" => app.show_help(),
        "toggle_history" => {
            let was_showing = app.show_history;
            app.toggle_history();
            let _ = resize_for_history(&webview, &saved_height, was_showing, app.show_history);
        }
        "edit_char" => {
            if let Some(ref p) = payload {
                if let InputMode::Editing(idx) = app.input_mode {
                    if let Some(c) = p.chars().next() {
                        app.edit_char(c, idx);
                    }
                }
            }
        }
        "edit_backspace" => {
            if let InputMode::Editing(idx) = app.input_mode {
                app.edit_backspace(idx);
            }
        }
        "clear_sound" => {
            app.sound_pending = false;
        }
        _ => {}
    }
    snapshot(&mut app)
}

/// Compact height: just title + timer + tasks + action bar (no history).
const COMPACT_HEIGHT: f64 = 280.0;

fn resize_for_history(
    webview: &WebviewWindow,
    saved_height: &State<'_, SavedWindowHeight>,
    was_showing: bool,
    now_showing: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if was_showing && !now_showing {
        let scale = webview.scale_factor()?;
        let outer = webview.outer_size()?;
        let current_height = outer.height as f64 / scale;
        *saved_height.0.lock().unwrap() = Some(current_height);
        webview.set_size(LogicalSize::new(outer.width as f64 / scale, COMPACT_HEIGHT))?;
    } else if !was_showing && now_showing {
        let mut saved = saved_height.0.lock().unwrap();
        if let Some(h) = saved.take() {
            let scale = webview.scale_factor()?;
            let outer = webview.outer_size()?;
            webview.set_size(LogicalSize::new(outer.width as f64 / scale, h))?;
        }
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(App::new())))
        .manage(SavedWindowHeight(Mutex::new(None)))
        .manage(TrayHolder(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![get_state, tick, action])
        .setup(|app| {
            let toggle_timer =
                MenuItemBuilder::with_id("toggle_timer", "Start / Pause").build(app)?;
            let show_hide =
                MenuItemBuilder::with_id("show_hide", "Show / Hide").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&toggle_timer)
                .item(&show_hide)
                .separator()
                .item(&quit)
                .build()?;

            let icon = Image::from_bytes(include_bytes!("../icons/32x32.png"))?;
            let tray = TrayIconBuilder::with_id("main-tray")
                .icon(icon)
                .menu(&menu)
                .title("\u{25cb} 60m")
                .tooltip("One Good Hour")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "toggle_timer" => {
                        let state = app.state::<AppState>();
                        let mut a = state.0.lock().unwrap();
                        a.toggle_timer();
                    }
                    "show_hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // Must keep tray alive — dropping it removes it from the menu bar
            let holder = app.state::<TrayHolder>();
            *holder.0.lock().unwrap() = Some(tray);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
