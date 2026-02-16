use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    symbols,
    text::Span,
    widgets::Widget,
};

use super::colors;

pub(crate) struct GradientBar {
    pub(crate) ratio: f64,
    pub(crate) label: String,
}

impl Widget for GradientBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bar_bg = colors::bar_bg();

        // Fill entire area with background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf[(x, y)].set_symbol(" ").set_bg(bar_bg);
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
            let color = colors::gradient_color(bar_position(x));
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
                cell.set_fg(Color::White).set_bg(bar_bg);
            } else {
                // Over filled region: dark text on gradient bg
                cell.set_fg(colors::label_text())
                    .set_bg(colors::gradient_color(bar_position(label_start + i as u16)));
            }
        }
    }
}
