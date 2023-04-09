use crate::{
    cursor::{Cursor, CursorDirection},
    draw_command::DrawCommand,
    rect::Rect,
    style::Style,
    vec::Vec2,
};

#[derive(Default)]
pub struct WidgetBuilder {
    pub commands: Vec<DrawCommand>,
    pub cursor: Cursor,
    pub panel_rect: Rect,
}

impl WidgetBuilder {
    pub fn new(panel_rect: Rect) -> Self {
        Self {
            commands: vec![],
            cursor: Cursor {
                position: (panel_rect.left, panel_rect.bottom).into(),
                direction: CursorDirection::Down,
            },
            panel_rect,
        }
    }

    pub fn push_command(&mut self, command: DrawCommand) {
        self.commands.push(command);
    }

    pub fn get_available_space(&self) -> Vec2 {
        Vec2::new(
            self.panel_rect.right - self.cursor.position.x,
            self.cursor.position.y - self.panel_rect.bottom,
        )
    }
}

pub trait Widget {
    fn get_style(&self) -> &Style;
    fn get_style_mut(&mut self) -> &mut Style;

    fn build(&self, builder: &mut WidgetBuilder);
}
