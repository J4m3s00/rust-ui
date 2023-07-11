use rust_graphics::{draw_command::DrawCommand, rect::Rect};

use crate::gui::{
    app::app::FontManager,
    svg::svg::Svg,
    widget::{state::observable::Observer, theme::theme::Theme},
};

use super::Drawable;

impl Drawable for Observer<Svg> {
    fn draw(&self, area: Rect, _font_manager: &FontManager, _theme: &Theme) -> Vec<DrawCommand> {
        self.get()
            .map(|svg| svg.generate_draw_commands(area))
            .unwrap_or_default()
    }
}
