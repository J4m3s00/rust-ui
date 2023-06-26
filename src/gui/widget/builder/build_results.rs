use crate::gui::widget::state::Observable;

use super::text::Text;

#[derive(Debug, Clone, Default)]
pub struct BuildResult {
    text: Option<Observable<Text>>,
    // Add render commands here as well in the future. need to work with scissor test
}

impl BuildResult {
    pub fn with_text(mut self, text: Observable<Text>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn text(&self) -> Option<Text> {
        self.text.as_ref()?.get()
    }
}
