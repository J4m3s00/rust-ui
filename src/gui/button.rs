use super::{
    label::Label,
    widget::{SizePolicy, SizePolicy2D, Widget},
    widget_builder::{WidgetBuilder, WidgetInteractionType},
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
    fn build(&self, builder: &mut WidgetBuilder, size: SizePolicy2D) {
        let _ = builder.push_child(size);
        self.label.build(builder, SizePolicy::Fill.into()); // Probaly change sizing policy here
        builder.interaction(WidgetInteractionType::Click);
        println!("Pop Child");
    }
}
