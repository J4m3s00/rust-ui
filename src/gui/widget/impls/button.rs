use rand::seq::SliceRandom;

use crate::{
    actions::receiver::Receiver,
    gui::widget::{
        builder::{build_context::BuildContext, build_results::BuildResult},
        state::State,
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

const RANDOM_STRINGS: [&str; 5] = [
    "Hallo Feli.",
    "Ich Liebe Dich.",
    "Hier und Da",
    "Sogar Richtig",
    "Oder Nichtig",
];

impl Widget for Button {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default().with_text(self.label.observe())
    }

    fn on_click(&self) {
        let random_string = RANDOM_STRINGS
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        let mut new_text = self.label.get();
        new_text.set_text(random_string);
        self.label.set(new_text);
        self.click_callback.action(ButtonClick)
    }

    fn on_mouse_enter(&self) {
        println!("Mouse enter");
    }

    fn on_mouse_leave(&self) {
        println!("Mouse leave");
    }
}
