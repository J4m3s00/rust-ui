use crate::{
    cursor::{Cursor, CursorDirection},
    style::Style,
    vec::Vec2,
    widget::{Widget, WidgetBuild},
};

pub struct VStack {
    pub children: Vec<Box<dyn Widget>>,
    pub style: Style,
}

impl Widget for VStack {
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build(&self, cursor: &Cursor) -> WidgetBuild {
        let mut commands = vec![];
        let mut cursor = cursor.clone();
        cursor.direction = CursorDirection::Down;

        for child in &self.children {
            let build = child.build(&cursor);
            commands.extend(build.commands);
            cursor.position = build.cursor;
        }

        WidgetBuild {
            commands,
            cursor: cursor.position,
        }
    }
}
