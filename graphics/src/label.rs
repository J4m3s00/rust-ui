use crate::{
    cursor::Cursor,
    draw_command::DrawCommand,
    get_current_font_size,
    style::Style,
    text::Text,
    vec::Vec2,
    widget::{Widget, WidgetBuild},
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

    fn build(&self, cursor: &Cursor) -> WidgetBuild {
        WidgetBuild {
            commands: vec![DrawCommand::Text(Text::new(
                cursor.position,
                self.text.clone(),
            ))],
            cursor: cursor.position + cursor.direction.get_default_step(),
        }
    }
}
