use crate::{
    draw_command::DrawCommand,
    style::Style,
    text::Text,
    widget::{Widget, WidgetBuilder},
};

pub struct Label {
    pub text: String,
    pub style: Style,
}

impl Widget for Label {
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
        builder.cursor.position += builder.cursor.direction.get_default_step();
    }
}
