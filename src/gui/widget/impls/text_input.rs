use crate::prelude::*;

pub struct TextInput {
    value: State<Text>,
    cursor_pos: State<usize>,
}

impl TextInput {
    pub fn new(init_value: impl Into<String>) -> WidgetInstance {
        Self {
            value: State::new(Text::from(init_value).hor_align(AlignH::Left)),
            cursor_pos: State::new(0),
        }
        .instance()
        .accept_input()
    }
}

impl Widget for TextInput {
    fn build(
        &mut self,
        _ctx: &mut BuildContext,
        mouse_state: Observer<WidgetMouseState>,
        focused: Observer<bool>,
    ) -> BuildResult {
        let mut res = BuildResult::default();
        res.draw_rect(DrawRect::fill_and_stroke(
            mouse_state.map(|s| match s {
                _ => None,
            }),
            None.into(),
        ));
        res.draw_text(self.value.observe());

        res
    }

    fn on_text_input(&self, input: String, _app_interface: AppInterface) {
        let mut text = self.value.get().clone();
        text.text.insert_str(self.cursor_pos.get(), &input);
        self.value.set(text);
        self.cursor_pos.set(self.cursor_pos.get() + input.len());
    }

    fn on_mouse_down(&self, event: &MouseEvent, _interface: AppInterface) {
        if event.inside {
            self.on_gain_focus();
        }
    }

    fn on_gain_focus(&self) {
        self.cursor_pos.set(self.value.get().text.len())
    }
}
