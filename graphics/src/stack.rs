use crate::{
    color::Color,
    cursor::CursorDirection,
    draw_command::{DrawCommand, Fill},
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
        let content_region = builder.get_content_region_available();
        builder.push_command(DrawCommand::Rect {
            left: content_region.left,
            bottom: content_region.bottom,
            width: content_region.width(),
            height: content_region.height(),
            fill: Some(Fill {
                color: Color::new(200, 200, 200, 255),
            }),
            stroke: None,
        });
        for child in &self.children {
            child.build(builder);
        }
    }
}
