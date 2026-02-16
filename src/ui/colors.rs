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

// Static color helpers for named colors used across the UI
pub(crate) fn bar_bg() -> Color {
    rgb_or_indexed(45, 45, 55, 236)
}

pub(crate) fn label_text() -> Color {
    rgb_or_indexed(30, 30, 30, 234)
}

pub(crate) fn selection_bg() -> Color {
    rgb_or_indexed(30, 30, 50, 235)
}

// Gradient colors as tuples (for lerp) with their 256-color fallbacks
const GRADIENT_GREEN: (u8, u8, u8) = (72, 199, 142);
const GRADIENT_YELLOW: (u8, u8, u8) = (250, 204, 21);
const GRADIENT_RED: (u8, u8, u8) = (239, 68, 68);

fn lerp_color(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f64) -> Color {
    let r = (c1.0 as f64 + (c2.0 as f64 - c1.0 as f64) * t) as u8;
    let g = (c1.1 as f64 + (c2.1 as f64 - c1.1 as f64) * t) as u8;
    let b = (c1.2 as f64 + (c2.2 as f64 - c1.2 as f64) * t) as u8;
    Color::Rgb(r, g, b)
}

/// Returns a gradient color for a position in [0.0, 1.0] (green → yellow → red).
/// Uses smooth RGB interpolation when truecolor is available, otherwise steps
/// through 256-color indexed values.
pub(crate) fn gradient_color(position: f64) -> Color {
    if supports_truecolor() {
        if position < 0.5 {
            lerp_color(GRADIENT_GREEN, GRADIENT_YELLOW, position * 2.0)
        } else {
            lerp_color(GRADIENT_YELLOW, GRADIENT_RED, (position - 0.5) * 2.0)
        }
    } else {
        gradient_color_indexed(position)
    }
}

/// Stepped gradient using 256-color indexed palette: green → yellow → red.
fn gradient_color_indexed(position: f64) -> Color {
    // 256-color steps: greens → yellows → reds
    const STEPS: &[u8] = &[
        79,  // green (bright)
        78,  // green
        114, // green-yellow
        150, // yellow-green
        186, // light yellow
        220, // yellow
        214, // orange-yellow
        208, // orange
        203, // red-orange
        196, // red
    ];
    let idx = (position * (STEPS.len() - 1) as f64).round() as usize;
    Color::Indexed(STEPS[idx.min(STEPS.len() - 1)])
}
