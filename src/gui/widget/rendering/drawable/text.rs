use rust_graphics::{
    color::{Color, COLOR_BLACK},
    draw_command::{DrawCommand, Stroke},
    font::Font,
    rect::Rect,
};

use crate::{
    gui::{
        app::app::FontManager,
        widget::{state::observable::Observer, theme::theme::Theme},
    },
    prelude::{AlignH, AlignV},
};

use super::Drawable;

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub font: Option<Font>,
    pub color: Color,
    pub outline: Option<Stroke>,
    pub alignment_h: AlignH,
    pub alignment_v: AlignV,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::new(),
            font: None,
            color: COLOR_BLACK,
            alignment_h: AlignH::Center,
            alignment_v: AlignV::Center,
            outline: None,
        }
    }
}

impl<T> From<T> for Text
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            ..Default::default()
        }
    }
}

impl Text {
    pub fn vert_align(mut self, alignment: AlignV) -> Self {
        self.alignment_v = alignment;
        self
    }

    pub fn hor_align(mut self, alignment: AlignH) -> Self {
        self.alignment_h = alignment;
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}

pub struct DrawText(pub Observer<Text>);

impl Drawable for DrawText {
    fn draw(&self, area: Rect, font_manager: &FontManager, _theme: &Theme) -> DrawCommand {
        let Some(text) = self.0.get() else {
            println!("Failed to observe text of draw text!");
            return DrawCommand::Line { x1: 0., y1: 0., x2: 0., y2: 0., stroke: Stroke {color: COLOR_BLACK, width: 0.} };
        };
        let line_top = font_manager.default_font().get_line_top() as f32;
        let line_bottom = font_manager.default_font().get_line_bottom() as f32;
        let text_width = font_manager.default_font().get_text_width(&text.text) as f32;
        let text_height = font_manager.default_font().get_text_height(&text.text) as f32;

        let text_base_line = match text.alignment_v {
            AlignV::Top => area.top + line_top,
            AlignV::Center => area.center().y + line_bottom + (text_height / 2.),
            AlignV::Bottom => area.bottom + line_bottom,
        };
        let text_left = match text.alignment_h {
            AlignH::Left => area.left,
            AlignH::Center => area.center().x - (text_width / 2.),
            AlignH::Right => area.right - text_width,
        };
        DrawCommand::Text {
            font: font_manager.default_font().clone(),
            text: text.text.clone(),
            position: (text_left, text_base_line).into(),
            color: text.color,
            stroke: None,
        }
    }
}
