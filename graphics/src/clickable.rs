use crate::{
    color::Color,
    draw_command::{DrawCommand, Fill, Stroke},
    style::Style,
    widget::Widget,
};

#[derive(Default)]
pub struct Clickable {
    style: Style,
}

impl Widget for Clickable {
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build(&self, builder: &mut crate::widget::WidgetBuilder) {
        let content_region = builder.get_content_region_available();
        builder.push_command(DrawCommand::Rect {
            left: content_region.left,
            bottom: content_region.bottom,
            width: content_region.width(),
            height: content_region.height(),
            fill: Some(Fill {
                color: Color::new(0, 255, 0, 255),
            }),
            stroke: Some(Stroke {
                width: 10.,
                color: Color::new(0, 0, 0, 255),
            }),
        });
    }
}
