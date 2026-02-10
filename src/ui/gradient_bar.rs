use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    symbols,
    text::Span,
    widgets::Widget,
};

const COLOR_GRADIENT_GREEN: (u8, u8, u8) = (72, 199, 142);
const COLOR_GRADIENT_YELLOW: (u8, u8, u8) = (250, 204, 21);
const COLOR_GRADIENT_RED: (u8, u8, u8) = (239, 68, 68);
const COLOR_BAR_BG: Color = Color::Rgb(45, 45, 55);
const COLOR_LABEL_TEXT: Color = Color::Rgb(30, 30, 30);

fn lerp_color(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f64) -> Color {
    let r = (c1.0 as f64 + (c2.0 as f64 - c1.0 as f64) * t) as u8;
    let g = (c1.1 as f64 + (c2.1 as f64 - c1.1 as f64) * t) as u8;
    let b = (c1.2 as f64 + (c2.2 as f64 - c1.2 as f64) * t) as u8;
    Color::Rgb(r, g, b)
}

fn gradient_color(position: f64) -> Color {
    if position < 0.5 {
        lerp_color(COLOR_GRADIENT_GREEN, COLOR_GRADIENT_YELLOW, position * 2.0)
    } else {
        lerp_color(COLOR_GRADIENT_YELLOW, COLOR_GRADIENT_RED, (position - 0.5) * 2.0)
    }
}

pub(crate) struct GradientBar {
    pub(crate) ratio: f64,
    pub(crate) label: String,
}

impl Widget for GradientBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Fill entire area with background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf[(x, y)].set_symbol(" ").set_bg(COLOR_BAR_BG);
            }
        }

        if area.width == 0 || area.height == 0 {
            return;
        }

        let bar_position = |x: u16| -> f64 {
            if area.width <= 1 {
                0.0
            } else {
                x as f64 / (area.width - 1) as f64
            }
        };

        let filled_end = (self.ratio * area.width as f64).round() as u16;
        let y = area.top();

        // Render filled cells with gradient
        for x in 0..filled_end {
            let color = gradient_color(bar_position(x));
            buf[(area.left() + x, y)]
                .set_symbol(symbols::block::FULL)
                .set_fg(color);
        }

        // Render label centered over the bar
        let label_start = area
            .width
            .saturating_sub(self.label.len() as u16)
            / 2;
        let label_span = Span::raw(&self.label);
        buf.set_span(area.left() + label_start, y, &label_span, self.label.len() as u16);

        // Swap fg/bg for label cells that overlap the filled region for contrast
        for (i, _) in self.label.chars().enumerate() {
            let x = area.left() + label_start + i as u16;
            if x >= area.right() {
                break;
            }
            let cell = &mut buf[(x, y)];
            if label_start + i as u16 >= filled_end {
                // Over unfilled region: light text on dark bg
                cell.set_fg(Color::White).set_bg(COLOR_BAR_BG);
            } else {
                // Over filled region: dark text on gradient bg
                cell.set_fg(COLOR_LABEL_TEXT)
                    .set_bg(gradient_color(bar_position(label_start + i as u16)));
            }
        }
    }
}
