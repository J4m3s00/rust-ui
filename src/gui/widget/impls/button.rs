use rust_graphics::{color::Color, draw_command::Fill};

use crate::{
    actions::receiver::Receiver,
    gui::widget::{
        builder::{
            build_context::BuildContext,
            build_results::{BuildResult, WidgetRenderItem, WidgetRenderRect},
        },
        state::{observable::Observer, state::State},
        widget::ToInstance,
        widget_instance::WidgetInstance,
    },
    prelude::{Text, Widget},
};

pub struct ButtonClick;

#[derive(Clone, Copy)]
enum ButtonMouseState {
    Normal,
    Hovered,
    Pressed,
}

pub struct Button {
    label: State<Text>,
    click_callback: Box<dyn Receiver<ButtonClick>>,
    hovered: State<ButtonMouseState>,
}

impl Button {
    pub fn new<T>(label: impl Into<String>, on_click: T) -> WidgetInstance
    where
        T: Receiver<ButtonClick> + 'static,
    {
        Self {
            label: State::new(Text::from(label)),
            click_callback: Box::new(on_click),
            hovered: State::new(ButtonMouseState::Normal),
        }
        .instance()
    }
}

impl Widget for Button {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default()
            .with_render_item(WidgetRenderItem::Rect(WidgetRenderRect {
                width: Observer::value_default(),
                height: Observer::value_default(),
                fill: self.hovered.map(|hovered| match *hovered {
                    ButtonMouseState::Normal => Some(Fill {
                        color: Color::new(64, 64, 64, 255),
                    }),
                    ButtonMouseState::Hovered => Some(Fill {
                        color: Color::new(128, 128, 128, 255),
                    }),
                    ButtonMouseState::Pressed => Some(Fill {
                        color: Color::new(255, 255, 255, 255),
                    }),
                }),
                stroke: None.into(),
            }))
            .with_text(self.label.observe())
    }

    fn on_mouse_down(&self) {
        self.hovered.set(ButtonMouseState::Pressed);
    }

    fn on_mouse_up(&self) {
        if let ButtonMouseState::Pressed = self.hovered.get() {
            self.click_callback.action(ButtonClick);
            self.hovered.set(ButtonMouseState::Hovered);
        }
    }

    fn on_mouse_enter(&self) {
        self.hovered.set(ButtonMouseState::Hovered);
    }

    fn on_mouse_leave(&self) {
        self.hovered.set(ButtonMouseState::Normal);
    }
}
