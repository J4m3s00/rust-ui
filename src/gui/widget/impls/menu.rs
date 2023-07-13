use crate::prelude::*;

pub struct Menu {
    items: Vec<WidgetInstance>,
}

impl Menu {
    pub fn new(children: Vec<WidgetInstance>) -> Self {
        Self { items: children }
    }
}

impl Widget for Menu {
    fn build(
        &mut self,
        _ctx: &mut BuildContext,
        _: Observer<WidgetMouseState>,
        _: Observer<bool>,
    ) -> BuildResult {
        let res = BuildResult::default();
        res
    }

    fn children(&self) -> &[WidgetInstance] {
        &self.items
    }
}
