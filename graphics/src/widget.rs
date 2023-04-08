use crate::{cursor::Cursor, draw_command::DrawCommand, style::Style, vec::Vec2};

pub struct WidgetBuild {
    pub commands: Vec<DrawCommand>,
    pub cursor: Vec2,
}

pub trait Widget {
    fn get_style(&self) -> &Style;
    fn get_style_mut(&mut self) -> &mut Style;

    fn build(&self, cursor: &Cursor) -> WidgetBuild;
}
