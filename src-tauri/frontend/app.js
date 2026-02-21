const { invoke } = window.__TAURI__.core;

let currentState = null;

// --- Themes ---
const CSS_VARS = [
    "bg", "bright", "normal", "dim", "faint",
    "selection-bg", "bar-bg", "label-text",
    "gradient-start", "gradient-mid", "gradient-end",
    "tasks-border", "history-border",
];

const THEMES = [
    {
        id: "ember",
        name: "Ember",
        swatches: ["#ffa000", "#c87800", "#784600"],
        colors: {
            "bg": "#0a0500",
            "bright": "#ffa000",
            "normal": "#c87800",
            "dim": "#784600",
            "faint": "#462800",
            "selection-bg": "#281600",
            "bar-bg": "#1e1000",
            "label-text": "#140a00",
            "gradient-start": "#ffb400",
            "gradient-mid": "#c86400",
            "gradient-end": "#8c3200",
            "tasks-border": "#c87800",
            "history-border": "#c87800",
        },
    },
    {
        id: "catppuccin",
        name: "Catppuccin",
        swatches: ["#f38ba8", "#74c7ec", "#cba6f7"],
        colors: {
            "bg": "#11111b",
            "bright": "#f38ba8",
            "normal": "#cdd6f4",
            "dim": "#a6adc8",
            "faint": "#45475a",
            "selection-bg": "#313244",
            "bar-bg": "#181825",
            "label-text": "#11111b",
            "gradient-start": "#f38ba8",
            "gradient-mid": "#89b4fa",
            "gradient-end": "#94e2d5",
            "tasks-border": "#74c7ec",
            "history-border": "#cba6f7",
        },
    },
    {
        id: "solarized",
        name: "Solarized",
        swatches: ["#268bd2", "#2aa198", "#859900"],
        colors: {
            "bg": "#002b36",
            "bright": "#268bd2",
            "normal": "#839496",
            "dim": "#586e75",
            "faint": "#073642",
            "selection-bg": "#073642",
            "bar-bg": "#001e26",
            "label-text": "#002b36",
            "gradient-start": "#268bd2",
            "gradient-mid": "#2aa198",
            "gradient-end": "#859900",
            "tasks-border": "#2aa198",
            "history-border": "#6c71c4",
        },
    },
    {
        id: "gruvbox",
        name: "Gruvbox",
        swatches: ["#fe8019", "#fabd2f", "#b8bb26"],
        colors: {
            "bg": "#1d2021",
            "bright": "#fe8019",
            "normal": "#ebdbb2",
            "dim": "#a89984",
            "faint": "#504945",
            "selection-bg": "#3c3836",
            "bar-bg": "#282828",
            "label-text": "#1d2021",
            "gradient-start": "#fb4934",
            "gradient-mid": "#fe8019",
            "gradient-end": "#fabd2f",
            "tasks-border": "#83a598",
            "history-border": "#d3869b",
        },
    },
];

let themeModalOpen = false;
let themeSelectedIndex = 0;
let currentThemeId = localStorage.getItem("1gh-theme") || "ember";

// --- Tick loop ---
setInterval(async () => {
    try {
        const s = await invoke("tick");
        render(s);
    } catch (e) {
        console.error("tick error:", e);
    }
}, 250);

// --- Action helper ---
async function sendAction(name, payload) {
    try {
        const s = await invoke("action", { name, payload: payload || null });
        render(s);
    } catch (e) {
        console.error("action error:", e);
    }
}

// --- Keyboard handler ---
document.addEventListener("keydown", handleKey);

function handleKey(e) {
    if (themeModalOpen) {
        handleThemeKey(e);
        return;
    }

    if (!currentState) return;

    const mode = currentState.input_mode;

    if (mode === "normal") {
        handleNormalKey(e);
    } else if (mode && mode.startsWith("editing:")) {
        handleEditingKey(e);
    } else if (mode === "modal") {
        handleModalKey(e);
    }
}

function handleNormalKey(e) {
    const key = e.key;
    let handled = true;

    switch (key) {
        case " ":
            sendAction("toggle_timer");
            break;
        case "r":
            sendAction("reset_timer");
            break;
        case "j":
        case "ArrowDown":
            sendAction("move_down");
            break;
        case "k":
        case "ArrowUp":
            sendAction("move_up");
            break;
        case "Enter":
            sendAction("start_editing");
            break;
        case "x":
            sendAction("toggle_todo");
            break;
        case "d":
            sendAction("remove_todo");
            break;
        case "c":
            sendAction("complete_session");
            break;
        case "h":
        case "ArrowLeft":
            sendAction("prev_history");
            break;
        case "l":
        case "ArrowRight":
            sendAction("next_history");
            break;
        case "y":
            sendAction("copy_markdown");
            break;
        case "D":
            sendAction("clear_notes");
            break;
        case "N":
            sendAction("new_session");
            break;
        case "?":
            sendAction("show_help");
            break;
        case "H":
            sendAction("toggle_history");
            break;
        case "t":
            openThemeModal();
            break;
        default:
            handled = false;
    }

    if (handled) e.preventDefault();
}

function handleEditingKey(e) {
    const key = e.key;
    let handled = true;

    switch (key) {
        case "Enter":
        case "Escape":
            sendAction("stop_editing");
            break;
        case "Backspace":
            sendAction("edit_backspace");
            break;
        default:
            if (key.length === 1) {
                sendAction("edit_char", key);
            } else {
                handled = false;
            }
    }

    if (handled) e.preventDefault();
}

function handleModalKey(e) {
    const key = e.key;
    let handled = true;

    switch (key) {
        case "y":
        case "Enter":
            sendAction("confirm_modal");
            break;
        case "n":
        case "Escape":
            sendAction("dismiss_modal");
            break;
        default:
            handled = false;
    }

    if (handled) e.preventDefault();
}

// --- Render ---
function render(state) {
    if (!state) return;
    currentState = state;

    renderTimer(state);
    renderTodos(state);
    renderActionBar(state);
    renderHistory(state);
    renderModal(state);

    if (state.sound_pending) {
        playBeep();
        sendAction("clear_sound");
    }
}

function renderTimer(state) {
    const fill = document.getElementById("progress-fill");
    const label = document.getElementById("progress-label");
    const status = document.getElementById("timer-status");

    const pct = Math.round(state.progress * 100);
    fill.style.width = (state.progress * 100) + "%";
    label.textContent = state.timer_display + " \u00b7 " + pct + "%";

    let statusIcon;
    if (state.is_running) {
        statusIcon = "\u25b6 Running";
    } else if (state.time_left === 0) {
        statusIcon = "\u2713 Done";
    } else {
        statusIcon = "\u23f8 Paused";
    }
    status.textContent = statusIcon + "  [Space] Play/Pause  [r] Reset";
}

function renderTodos(state) {
    for (let i = 0; i < 4; i++) {
        const row = document.getElementById("todo-" + i);
        const todo = state.todos[i];
        const isSelected = i === state.selected_todo;
        const isEditing = state.editing_index === i;

        // Selection highlight
        row.className = "todo-row" + (isSelected ? " selected" : "");

        // Selector arrow
        const selector = row.querySelector(".todo-selector");
        selector.textContent = isSelected ? "\u25b8 " : "  ";

        // Checkbox
        const checkbox = row.querySelector(".todo-checkbox");
        checkbox.textContent = todo.completed ? "[x] " : "[ ] ";
        checkbox.className = "todo-checkbox" + (todo.completed ? " completed" : "");

        // Text
        const textEl = row.querySelector(".todo-text");
        if (isEditing) {
            textEl.innerHTML = escapeHtml(todo.text) + '<span class="cursor-char">\u258e</span>';
            textEl.className = "todo-text editing";
        } else if (todo.text === "") {
            textEl.textContent = "(empty)";
            textEl.className = "todo-text empty";
        } else if (todo.completed) {
            textEl.textContent = todo.text;
            textEl.className = "todo-text completed";
        } else {
            textEl.textContent = todo.text;
            textEl.className = "todo-text";
        }
    }
}

function renderActionBar(state) {
    const bar = document.getElementById("action-bar-text");
    if (state.status_message) {
        bar.innerHTML = '<span class="key-hint">' + escapeHtml(state.status_message) + '</span>';
    } else {
        bar.innerHTML =
            '<span class="key-hint">[x]</span> Check  ' +
            '<span class="key-hint">[c]</span> Complete  ' +
            '<span class="key-hint">[N]</span> New  ' +
            '<span class="key-hint">[t]</span> Themes  ' +
            '<span class="key-hint">[?]</span> Help';
    }
}

function renderHistory(state) {
    const section = document.getElementById("history-section");
    const empty = document.getElementById("history-empty");
    const header = document.getElementById("history-header");
    const todosEl = document.getElementById("history-todos");
    const footer = document.getElementById("history-footer");

    if (!state.show_history) {
        section.classList.add("hidden");
        return;
    }
    section.classList.remove("hidden");

    if (!state.completed_notes || state.completed_notes.length === 0) {
        empty.classList.remove("hidden");
        header.classList.add("hidden");
        todosEl.innerHTML = "";
        footer.classList.add("hidden");
        return;
    }

    empty.classList.add("hidden");
    header.classList.remove("hidden");
    footer.classList.remove("hidden");

    const idx = state.history_index || 0;
    const total = state.history_total || state.completed_notes.length;
    const note = state.completed_notes[idx];

    header.innerHTML =
        '<span style="color:var(--dim)">[←/h] </span>' +
        '<span style="color:var(--bright);font-weight:bold">Session ' + (idx + 1) + ' of ' + total + '</span>' +
        '<span style="color:var(--dim)"> [→/l]</span>';

    todosEl.innerHTML = "";
    for (const todo of note.todos) {
        if (todo.text === "") continue;
        const div = document.createElement("div");
        div.className = "history-todo";
        const check = todo.completed ? "[x]" : "[ ]";
        const checkClass = todo.completed ? "todo-checkbox completed" : "todo-checkbox";
        const textClass = todo.completed ? "todo-text completed" : "todo-text";
        div.innerHTML =
            '<span class="' + checkClass + '">  ' + check + ' </span>' +
            '<span class="' + textClass + '">' + escapeHtml(todo.text) + '</span>';
        todosEl.appendChild(div);
    }

    const timeDisplay = note.time_spent;
    footer.innerHTML =
        '<span style="color:var(--dim)">Time: ' + timeDisplay + '</span>  ' +
        '<span class="key-hint">[y]</span> ' +
        '<span style="color:var(--normal)">Copy  </span>' +
        '<span class="key-hint">[D]</span> ' +
        '<span style="color:var(--normal)">Clear</span>';
}

function renderModal(state) {
    const overlay = document.getElementById("modal-overlay");
    const title = document.getElementById("modal-title");
    const body = document.getElementById("modal-body");

    if (!state.modal) {
        overlay.classList.add("hidden");
        return;
    }

    overlay.classList.remove("hidden");

    if (state.modal === "help") {
        title.textContent = "Shortcuts";
        body.innerHTML = renderHelpContent();
    } else if (state.modal === "complete_session") {
        title.textContent = "Complete Session";
        body.innerHTML = 'Complete this session and save to history?\n\n<span class="key-hint">[y]</span> Yes  <span class="key-hint">[n]</span> No';
    } else if (state.modal === "clear_notes") {
        title.textContent = "Clear History";
        body.innerHTML = 'Clear all completed sessions?\n\n<span class="key-hint">[y]</span> Yes  <span class="key-hint">[n]</span> No';
    } else if (state.modal === "new_session") {
        title.textContent = "New Session";
        body.innerHTML = 'Start fresh? This clears all tasks and history.\n\n<span class="key-hint">[y]</span> Yes  <span class="key-hint">[n]</span> No';
    }
}

function renderHelpContent() {
    const shortcuts = [
        ["Space", "Play/Pause timer", "j/\u2193", "Move down"],
        ["r", "Reset timer", "k/\u2191", "Move up"],
        ["Enter", "Edit task", "x", "Check off task"],
        ["d", "Clear task", "c", "Complete session"],
        ["h/\u2190", "Prev history", "l/\u2192", "Next history"],
        ["y", "Copy markdown", "D", "Clear history"],
        ["N", "New session", "t", "Themes"],
        ["H", "Toggle history", "?", "Show help"],
    ];

    let html = '<div class="help-table">';
    for (const [k1, d1, k2, d2] of shortcuts) {
        html += '<div class="help-row">' +
            '<span class="help-key">' + escapeHtml(k1) + '</span>' +
            '<span class="help-desc">' + escapeHtml(d1) + '</span>' +
            '<span class="help-key">' + escapeHtml(k2) + '</span>' +
            '<span class="help-desc">' + escapeHtml(d2) + '</span>' +
            '</div>';
    }
    html += '</div>';
    html += '<div class="help-close">[Esc] Close</div>';
    return html;
}

// --- Theme modal ---
function openThemeModal() {
    themeModalOpen = true;
    themeSelectedIndex = THEMES.findIndex(t => t.id === currentThemeId);
    if (themeSelectedIndex < 0) themeSelectedIndex = 0;
    renderThemeModal();
    document.getElementById("theme-overlay").classList.remove("hidden");
}

function closeThemeModal() {
    themeModalOpen = false;
    document.getElementById("theme-overlay").classList.add("hidden");
}

function applyTheme(themeId) {
    currentThemeId = themeId;
    localStorage.setItem("1gh-theme", themeId);
    const theme = THEMES.find(t => t.id === themeId);
    if (!theme) return;
    const root = document.documentElement;
    for (const key of CSS_VARS) {
        if (theme.colors[key]) {
            root.style.setProperty("--" + key, theme.colors[key]);
        }
    }
}

function handleThemeKey(e) {
    const key = e.key;
    let handled = true;

    switch (key) {
        case "j":
        case "ArrowDown":
            themeSelectedIndex = (themeSelectedIndex + 1) % THEMES.length;
            renderThemeModal();
            break;
        case "k":
        case "ArrowUp":
            themeSelectedIndex = (themeSelectedIndex - 1 + THEMES.length) % THEMES.length;
            renderThemeModal();
            break;
        case "Enter":
            applyTheme(THEMES[themeSelectedIndex].id);
            renderThemeModal();
            break;
        case "Escape":
        case "t":
            closeThemeModal();
            break;
        default:
            handled = false;
    }

    if (handled) e.preventDefault();
}

function renderThemeModal() {
    const list = document.getElementById("theme-list");
    list.innerHTML = "";

    for (let i = 0; i < THEMES.length; i++) {
        const theme = THEMES[i];
        const isSelected = i === themeSelectedIndex;
        const isActive = theme.id === currentThemeId;

        const div = document.createElement("div");
        div.className = "theme-option" + (isSelected ? " selected" : "");

        const selector = '<span class="theme-selector">' + (isSelected ? "\u25b8" : " ") + '</span>';
        const name = '<span class="theme-name">' + escapeHtml(theme.name) + '</span>';
        const active = isActive ? '<span class="theme-active">\u2713</span>' : '';

        let swatches = '<span class="theme-swatch">';
        for (const color of theme.swatches) {
            swatches += '<span class="theme-swatch-dot" style="background:' + color + '"></span>';
        }
        swatches += '</span>';

        div.innerHTML = selector + name + active + swatches;
        list.appendChild(div);
    }
}

// --- Sound ---
function playBeep() {
    try {
        const ctx = new AudioContext();
        const osc = ctx.createOscillator();
        const gain = ctx.createGain();
        osc.type = "sine";
        osc.frequency.value = 800;
        gain.gain.value = 0.3;
        osc.connect(gain);
        gain.connect(ctx.destination);
        osc.start();
        gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 0.15);
        osc.stop(ctx.currentTime + 0.15);
    } catch (e) {
        console.warn("Audio not available:", e);
    }
}

// --- Utilities ---
function escapeHtml(str) {
    const div = document.createElement("div");
    div.textContent = str;
    return div.innerHTML;
}

function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return mins + ":" + String(secs).padStart(2, "0");
}

// --- Initial load ---
document.addEventListener("DOMContentLoaded", () => {
    applyTheme(currentThemeId);
    invoke("get_state").then(render).catch(e => console.error("init error:", e));
});
