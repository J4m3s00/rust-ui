use rust_graphics::rect::Rect;

use crate::{
    actions::{receiver::Receiver, signal::Signal},
    gui::widget::{
        build_context::BuildContext, build_results::BuildResult, widget::ToInstance,
        widget_instance::WidgetInstance,
    },
    prelude::Widget,
};

pub struct ButtonClick;

pub struct Button {
    label: String,
    on_click: Signal<ButtonClick>,
}

impl Button {
    pub fn new(label: impl Into<String> /*, receiver: Box<T> */) -> WidgetInstance
/*where
        T: Receiver + 'static,*/ {
        let mut res = Self {
            label: label.into(),
            on_click: Signal::default(),
        };
        //res.on_click.connect(receiver);
        res.instance()
    }
}

impl Widget for Button {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default().with_text(self.label.clone())
    }
}

impl Receiver for Button {
    //type Action = ButtonClick;

    fn action(self) {
        println!("Clicked!");
    }
}
