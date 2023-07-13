use crate::prelude::*;

pub struct TextInput {
    value: State<Text>,
}

impl TextInput {
    pub fn new(init_value: impl Into<String>) -> WidgetInstance {
        Self {
            value: State::new(Text::from(init_value).hor_align(AlignH::Left)),
        }
        .instance()
        .accept_input()
    }
}

impl Widget for TextInput {
    fn build(
        &mut self,
        _ctx: &mut BuildContext,
        _mouse_state: &State<WidgetMouseState>,
    ) -> BuildResult {
        let mut res = BuildResult::default();

        res.draw_text(self.value.observe());

        res
    }
}
