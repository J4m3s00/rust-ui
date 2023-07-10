use crate::{
    gui::widget::{
        builder::{build_context::BuildContext, build_results::BuildResult},
        widget::WidgetMouseState,
    },
    prelude::{State, Widget, WidgetInstance},
};

pub struct Menu {
    items: Vec<WidgetInstance>,
}

impl Menu {
    pub fn new(children: Vec<WidgetInstance>) -> Self {
        Self { items: children }
    }
}

impl Widget for Menu {
    fn build(&mut self, _ctx: &mut BuildContext, _: &State<WidgetMouseState>) -> BuildResult {
        let res = BuildResult::default();
        res
    }

    fn children(&self) -> &[WidgetInstance] {
        &self.items
    }
}
