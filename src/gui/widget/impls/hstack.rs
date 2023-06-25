use crate::{
    gui::widget::{
        build_context::{BuildContext, CursorDirection},
        build_results::BuildResult,
        widget::ToInstance,
    },
    prelude::{SizePolicy, Widget, WidgetInstance},
};

pub struct HStack {
    children: Vec<WidgetInstance>,
}

impl HStack {
    pub fn new(children: Vec<WidgetInstance>) -> WidgetInstance {
        Self { children }.instance()
    }
}

impl Widget for HStack {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        // Update context cursor direction for the children
        ctx.set_cursor_direction(CursorDirection::Horizontal);

        let content_area = ctx.get_content_rect().clone();
        let mut remaining_width = content_area.width();
        let mut total_frac = 0.;
        for item in self.children().iter() {
            match item.size().horizontal {
                SizePolicy::Fixed(pixels) => {
                    remaining_width -= pixels;
                }
                SizePolicy::Percentage(percent) => {
                    remaining_width -= percent * content_area.width();
                }
                SizePolicy::PercentageH(percent) => {
                    remaining_width -= percent * content_area.width();
                }
                SizePolicy::PercentageV(percent) => {
                    remaining_width -= percent * content_area.height();
                }
                SizePolicy::Fraction(frac) => {
                    total_frac += frac;
                }
            }
        }

        let frac_width = remaining_width / total_frac;

        for item in self.children.iter_mut() {
            let width = match item.size().horizontal {
                SizePolicy::Fixed(pixels) => pixels,
                SizePolicy::Percentage(percent) => percent * content_area.width(),
                SizePolicy::PercentageH(percent) => percent * content_area.width(),
                SizePolicy::PercentageV(percent) => percent * content_area.height(),
                SizePolicy::Fraction(frac) => frac * frac_width,
            };
            if let Some(mut child_context) =
                ctx.allocate_space((width, content_area.height()).into())
            {
                item.build(&mut child_context);
            }
        }
        BuildResult::default()
    }

    fn children(&self) -> &[WidgetInstance] {
        &self.children
    }
}
