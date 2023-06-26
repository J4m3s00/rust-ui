use crate::{
    gui::widget::{
        builder::{
            align::{AlignH, AlignV},
            build_context::BuildContext,
            build_results::BuildResult,
            text::Text,
        },
        state::State,
        widget::ToInstance,
        widget_instance::WidgetInstance,
    },
    prelude::Widget,
};

pub struct Label {
    text: State<Text>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> WidgetInstance {
        Self {
            text: State::new(Text::from(text.into())),
        }
        .instance()
    }

    pub fn new_v(text: impl Into<String>, alignment: AlignV) -> WidgetInstance {
        Self {
            text: State::new(Text::from(text.into()).vert_align(alignment)),
        }
        .instance()
    }

    pub fn new_h(text: impl Into<String>, alignment: AlignH) -> WidgetInstance {
        Self {
            text: State::new(Text::from(text.into()).hor_align(alignment)),
        }
        .instance()
    }

    pub fn new_a(text: impl Into<String>, alignment: (AlignH, AlignV)) -> WidgetInstance {
        Self {
            text: State::new(
                Text::from(text.into())
                    .hor_align(alignment.0)
                    .vert_align(alignment.1),
            ),
        }
        .instance()
    }
}

impl Widget for Label {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        BuildResult::default().with_text(self.text.observe())
    }
}
