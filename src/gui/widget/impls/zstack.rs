use crate::gui::widget::{
    builder::{
        build_context::{BuildContext, CursorDirection},
        build_results::BuildResult,
    },
    widget::{ToInstance, Widget},
    widget_instance::WidgetInstance,
};

pub struct ZStack {
    pub children: Vec<WidgetInstance>,
}

impl ZStack {
    pub fn new(children: Vec<WidgetInstance>) -> WidgetInstance {
        Self { children }.instance()
    }
}

impl Widget for ZStack {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        // Update context cursor direction for the children
        ctx.set_cursor_direction(CursorDirection::Stacked);

        //let mut ctx = ctx.clone();
        for item in self.children.iter_mut() {
            if let Some(mut child_context) = ctx.allocate_space(ctx.get_content_rect().size()) {
                //ctx = child_context.clone();
                item.build(&mut child_context);
            }
        }

        BuildResult::default()
    }

    fn children(&self) -> &[WidgetInstance] {
        &self.children
    }
}
