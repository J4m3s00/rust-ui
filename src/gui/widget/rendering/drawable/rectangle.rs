use rust_graphics::{
    draw_command::{DrawCommand, Fill, Stroke},
    rect::Rect,
};

use crate::gui::{app::app::FontManager, widget::state::observable::Observer};

use super::Drawable;

pub struct DrawRect {
    fill: Observer<Option<Fill>>,
    stroke: Observer<Option<Stroke>>,
}

impl DrawRect {
    pub fn fill(fill: Observer<Option<Fill>>) -> Self {
        Self {
            fill,
            stroke: Observer::value(None),
        }
    }

    pub fn stroke(stroke: Observer<Option<Stroke>>) -> Self {
        Self {
            fill: Observer::value(None),
            stroke,
        }
    }

    pub fn fill_and_stroke(fill: Observer<Option<Fill>>, stroke: Observer<Option<Stroke>>) -> Self {
        Self { fill, stroke }
    }
}

impl Drawable for DrawRect {
    fn draw(&self, area: Rect, _font_manager: &FontManager) -> DrawCommand {
        DrawCommand::Rect {
            left: area.left,
            top: area.top,
            width: area.width(),
            height: area.height(),
            fill: self.fill.get().unwrap_or(None),
            stroke: self.stroke.get().unwrap_or(None),
        }
    }
}
