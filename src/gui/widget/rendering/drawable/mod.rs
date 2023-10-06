pub mod draw_svg;
pub mod item;
pub mod rectangle;
pub mod text;

use rust_graphics::draw_command::DrawCommand;

use crate::{
    gui::{app::app::FontManager, widget::theme::theme::Theme},
    prelude::Rect,
};

pub trait Drawable {
    fn draw(&self, area: Rect, font_manager: &FontManager, theme: &Theme) -> Vec<DrawCommand>;
}
