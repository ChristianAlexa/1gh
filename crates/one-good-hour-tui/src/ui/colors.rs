use ratatui::style::Color;
use std::sync::OnceLock;

static TRUECOLOR: OnceLock<bool> = OnceLock::new();

fn supports_truecolor() -> bool {
    *TRUECOLOR.get_or_init(|| {
        std::env::var("COLORTERM")
            .map(|v| v == "truecolor" || v == "24bit")
            .unwrap_or(false)
    })
}

pub(crate) fn rgb_or_indexed(r: u8, g: u8, b: u8, indexed: u8) -> Color {
    if supports_truecolor() {
        Color::Rgb(r, g, b)
    } else {
        Color::Indexed(indexed)
    }
}

// ── Monochromatic orange palette ──

/// Bright orange — titles, active highlights, key hints
pub(crate) fn bright() -> Color {
    rgb_or_indexed(255, 160, 0, 214)
}

/// Standard orange — normal text, borders, checkboxes
pub(crate) fn normal() -> Color {
    rgb_or_indexed(200, 120, 0, 172)
}

/// Dim orange — secondary text, hints, inactive elements
pub(crate) fn dim() -> Color {
    rgb_or_indexed(120, 70, 0, 130)
}

/// Very dim — subtle separators, empty placeholders
pub(crate) fn faint() -> Color {
    rgb_or_indexed(70, 40, 0, 94)
}

/// Selection/highlight background
pub(crate) fn selection_bg() -> Color {
    rgb_or_indexed(40, 22, 0, 52)
}

/// Progress bar background
pub(crate) fn bar_bg() -> Color {
    rgb_or_indexed(30, 16, 0, 52)
}

/// Dark text for labels over bright bar fills
pub(crate) fn label_text() -> Color {
    rgb_or_indexed(20, 10, 0, 16)
}

// ── Gradient (bright orange → dim orange as time runs out) ──

const GRADIENT_BRIGHT: (u8, u8, u8) = (255, 180, 0);
const GRADIENT_MID: (u8, u8, u8) = (200, 100, 0);
const GRADIENT_DIM: (u8, u8, u8) = (140, 50, 0);

fn lerp_color(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f64) -> Color {
    let r = (c1.0 as f64 + (c2.0 as f64 - c1.0 as f64) * t) as u8;
    let g = (c1.1 as f64 + (c2.1 as f64 - c1.1 as f64) * t) as u8;
    let b = (c1.2 as f64 + (c2.2 as f64 - c1.2 as f64) * t) as u8;
    Color::Rgb(r, g, b)
}

/// Returns a gradient color for a position in [0.0, 1.0] (bright → mid → dim orange).
pub(crate) fn gradient_color(position: f64) -> Color {
    if supports_truecolor() {
        if position < 0.5 {
            lerp_color(GRADIENT_BRIGHT, GRADIENT_MID, position * 2.0)
        } else {
            lerp_color(GRADIENT_MID, GRADIENT_DIM, (position - 0.5) * 2.0)
        }
    } else {
        gradient_color_indexed(position)
    }
}

/// Stepped gradient using 256-color indexed palette.
fn gradient_color_indexed(position: f64) -> Color {
    const STEPS: &[u8] = &[
        214, // bright orange
        208, // orange
        172, // darker orange
        166, // deep orange
        130, // brown-orange
    ];
    let idx = (position * (STEPS.len() - 1) as f64).round() as usize;
    Color::Indexed(STEPS[idx.min(STEPS.len() - 1)])
}
