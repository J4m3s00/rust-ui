use crate::{
    actions::receiver::Receiver,
    gui::widget::{
        builder::{build_context::BuildContext, build_results::BuildResult},
        state::state::State,
        widget::ToInstance,
        widget_instance::WidgetInstance,
    },
    prelude::{Text, Widget},
};

pub struct ButtonClick;

pub struct Button {
    label: State<Text>,
    click_callback: Box<dyn Receiver<ButtonClick>>,
    hovered: State<bool>,
}

impl Button {
    pub fn new<T>(label: impl Into<String>, on_click: T) -> WidgetInstance
    where
        T: Receiver<ButtonClick> + 'static,
    {
        Self {
            label: State::new(Text::from(label)),
            click_callback: Box::new(on_click),
            hovered: State::new(false),
        }
        .instance()
    }
}

impl Widget for Button {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default().with_text(self.label.observe())
    }

    fn on_click(&self) {
        self.click_callback.action(ButtonClick)
    }

    fn on_mouse_enter(&self) {
        self.hovered.set(true);
    }

    fn on_mouse_leave(&self) {
        self.hovered.set(false);
    }
}
