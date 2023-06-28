use rust_graphics::{
    draw_command::{DrawCommand, Fill, Stroke},
    rect::Rect,
};

use crate::{
    gui::{
        app::app::FontManager,
        widget::{state::observable::Observer, theme::theme::Theme},
    },
    prelude::ColorId,
};

use super::Drawable;

pub struct DrawRect {
    fill: Observer<Option<ColorId>>,
    stroke: Observer<Option<(ColorId, f32)>>,
}

impl DrawRect {
    pub fn fill(fill: impl Into<Observer<Option<ColorId>>>) -> Self {
        Self {
            fill: fill.into(),
            stroke: Observer::value(None),
        }
    }

    pub fn stroke(stroke: impl Into<Observer<Option<(ColorId, f32)>>>) -> Self {
        Self {
            fill: Observer::value(None),
            stroke: stroke.into(),
        }
    }

    pub fn fill_and_stroke(
        fill: Observer<Option<ColorId>>,
        stroke: Observer<Option<(ColorId, f32)>>,
    ) -> Self {
        Self { fill, stroke }
    }
}

impl Drawable for DrawRect {
    fn draw(&self, area: Rect, _font_manager: &FontManager, theme: &Theme) -> DrawCommand {
        let fill = self.fill.get().unwrap().map(|id| Fill {
            color: theme.colors.from_id(id),
        });

        let stroke = self.stroke.get().unwrap().map(|(id, width)| Stroke {
            color: theme.colors.from_id(id),
            width,
        });

        DrawCommand::Rect {
            left: area.left,
            top: area.top,
            width: area.width(),
            height: area.height(),
            fill,
            stroke,
        }
    }
}
