use crate::{
    gui::widget::{
        builder::{build_context::BuildContext, build_results::BuildResult},
        rendering::drawable::text::DrawText,
        state::observable::Observer,
        widget::ToInstance,
        widget_instance::WidgetInstance,
    },
    prelude::{Text, Widget},
};

pub struct Label {
    text: Observer<Text>,
}

impl Label {
    pub fn new(text: impl Into<Text>) -> WidgetInstance {
        Self {
            text: Observer::value(text.into()),
        }
        .instance()
    }

    pub fn new_observe(text: impl Into<Observer<Text>>) -> WidgetInstance {
        Self { text: text.into() }.instance()
    }
}

impl Widget for Label {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        let mut res = BuildResult::default();
        res.draw_text(DrawText(Observer::reference(&self.text)));
        res
    }
}
