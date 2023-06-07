use super::{
    widget::{SizePolicy2D, Widget},
    widget_builder::PushChild,
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
    fn build(&self, builder: &PushChild, _size: SizePolicy2D) {
        builder.text(self.text.clone());
    }
}
