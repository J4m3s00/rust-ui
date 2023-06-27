pub mod item;
pub mod rectangle;
pub mod text;

use rust_graphics::{draw_command::DrawCommand, rect::Rect};

use crate::gui::app::app::FontManager;

pub trait Drawable {
    fn draw(&self, area: Rect, font_manager: &FontManager) -> DrawCommand;
}
