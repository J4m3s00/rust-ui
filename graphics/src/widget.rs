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
    pub content_rect_stack: Vec<Rect>,
}

impl WidgetBuilder {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            cursor: Cursor {
                position: (0., 0.).into(),
                direction: CursorDirection::Down,
            },
            content_rect_stack: vec![],
        }
    }

    pub fn push_command(&mut self, command: DrawCommand) {
        self.commands.push(command);
    }

    pub fn begin(&mut self, size: Vec2) {
        self.content_rect_stack.push(Rect::new(
            self.cursor.position.x,
            self.cursor.position.y,
            self.cursor.position.x + size.x,
            self.cursor.position.y - size.y,
        ));
    }

    pub fn end(&mut self) {
        self.content_rect_stack
            .pop()
            .expect("Invalid order of begin and end");
    }

    pub fn add_item(&mut self, size: Vec2) {}

    pub fn get_content_region_available(&self) -> Rect {
        self.content_rect_stack
            .last()
            .expect("Invalid order of begin and end")
            .clone()
            .set_top(self.cursor.position.y)
            .set_left(self.cursor.position.x)
    }
}

pub trait Widget {
    fn get_style(&self) -> &Style;
    fn get_style_mut(&mut self) -> &mut Style;

    fn build(&self, builder: &mut WidgetBuilder);
}
