use crate::{
    draw_command::DrawCommand,
    get_current_font_size,
    rect::Rect,
    style::Style,
    text::Text,
    widget::{Widget, WidgetBuilder},
};

pub struct Button {
    pub text: String,
    pub style: Style,
}

impl Widget for Button {
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build(&self, builder: &mut WidgetBuilder) {
        builder.commands.push(DrawCommand::Text(Text::new(
            builder.cursor.position,
            self.text.clone(),
        )));

        builder.commands.push(DrawCommand::Rect {
            left: builder.cursor.position.x,
            bottom: builder.cursor.position.y + get_current_font_size(),
            width: builder.get_content_region_available().width(),
            height: builder.cursor.position.y,
            fill: None,
            stroke: None,
        });
        builder.cursor.position += builder.cursor.direction.get_default_step();
    }
}
