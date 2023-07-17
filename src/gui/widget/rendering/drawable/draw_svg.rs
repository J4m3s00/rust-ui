use rust_graphics::{
    draw_command::{DrawCommand, Stroke},
    rect::Rect,
};

use crate::{
    gui::{
        app::app::FontManager,
        svg::svg::Svg,
        widget::{state::observable::Observer, theme::theme::Theme},
    },
    prelude::ColorId,
};

use super::Drawable;

pub struct DrawSvg {
    svg: Observer<Svg>,
    color: Option<Observer<ColorId>>,
}

impl DrawSvg {
    pub fn new(svg: impl Into<Observer<Svg>>) -> Self {
        Self {
            svg: svg.into(),
            color: None,
        }
    }

    pub fn new_color(svg: impl Into<Observer<Svg>>, color: impl Into<Observer<ColorId>>) -> Self {
        Self {
            svg: svg.into(),
            color: Some(color.into()),
        }
    }
}

impl Drawable for DrawSvg {
    fn draw(&self, area: Rect, _font_manager: &FontManager, theme: &Theme) -> Vec<DrawCommand> {
        self.svg
            .get()
            .map(|svg| {
                svg.generate_draw_commands(
                    area,
                    self.color
                        .as_ref()
                        .map(|c| c.get().map(|c| Stroke::new(theme.colors.from_id(c), 1.0)))
                        .unwrap_or(None),
                )
            })
            .unwrap_or_default()
    }
}
