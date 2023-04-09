use crate::{
    cursor::CursorDirection,
    style::Style,
    widget::{Widget, WidgetBuilder},
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

    fn build(&self, builder: &mut WidgetBuilder) {
        builder.cursor.direction = CursorDirection::Down;
        for child in &self.children {
            child.build(builder);
        }
    }
}
