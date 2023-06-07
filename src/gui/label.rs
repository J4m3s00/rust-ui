use super::{
    widget::{SizePolicy2D, Widget},
    widget_builder::WidgetBuilder,
};

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for Label {
    fn build(&self, builder: &mut WidgetBuilder, size: SizePolicy2D) {
        builder.child(size).text(self.text.clone());
    }
}
