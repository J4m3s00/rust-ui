use crate::prelude::*;

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
    fn build(&mut self, _ctx: &mut BuildContext, _: &State<WidgetMouseState>) -> BuildResult {
        let mut res = BuildResult::default();
        res.draw_text(self.text.clone());
        res
    }
}
