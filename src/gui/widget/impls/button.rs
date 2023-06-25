use crate::{
    actions::receiver::Receiver,
    gui::widget::{
        build_context::BuildContext, build_results::BuildResult, widget::ToInstance,
        widget_instance::WidgetInstance,
    },
    prelude::Widget,
};

pub struct ButtonClick;

pub struct Button {
    label: String,
    click_callback: Box<dyn Receiver<ButtonClick>>,
}

impl Button {
    pub fn new<T>(label: impl Into<String>, on_click: T) -> WidgetInstance
    where
        T: Receiver<ButtonClick> + 'static,
    {
        Self {
            label: label.into(),
            click_callback: Box::new(on_click),
        }
        .instance()
    }
}

impl Widget for Button {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default().with_text(self.label.clone())
    }

    fn on_click(&self) {
        self.click_callback.action(ButtonClick)
    }
}
