use crate::gui::widget::state::observable::Observer;

use super::text::Text;

#[derive(Default)]
pub struct BuildResult {
    text: Option<Observer<Text>>,
    // Add render commands here as well in the future. need to work with scissor test
}

impl BuildResult {
    pub fn with_text(mut self, text: Observer<Text>) -> Self {
        self.text = Some(text);
        self
    }

    pub fn text(&self) -> Option<Text> {
        self.text.as_ref()?.get()
    }
}
