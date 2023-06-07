use super::{
    label::Label,
    widget::{SizePolicy, SizePolicy2D, Widget},
    widget_builder::{PushChild, WidgetInteractionType},
};

pub struct Button {
    label: Label,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: Label::new(label.into()),
        }
    }
}

impl Widget for Button {
    fn build(&self, builder: &PushChild, size: SizePolicy2D) {
        builder.child(size, |child| {
            child
                .widget(&self.label, SizePolicy::Fill.into())
                .interaction(WidgetInteractionType::Click);
        });
    }
}
