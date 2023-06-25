use crate::{
    gui::widget::{
        build_context::{BuildContext, CursorDirection},
        build_results::BuildResult,
        size_policy::SizePolicy,
        widget::ToInstance,
    },
    prelude::{Widget, WidgetInstance},
};

pub struct VStack {
    children: Vec<WidgetInstance>,
}

impl VStack {
    pub fn new(children: Vec<WidgetInstance>) -> WidgetInstance {
        Self { children }.instance()
    }
}

impl Widget for VStack {
    fn build(&mut self, ctx: &mut BuildContext) -> BuildResult {
        // Update context cursor direction for the children
        ctx.set_cursor_direction(CursorDirection::Vertical);
        let content_area = ctx.get_content_rect().clone();
        let mut remaining_height = content_area.height();
        let mut total_frac = 0.;
        for item in self.children().iter() {
            match item.size().vertical {
                SizePolicy::Fixed(pixels) => {
                    remaining_height -= pixels;
                }
                SizePolicy::Percentage(percent) => {
                    remaining_height -= percent * content_area.height();
                }
                SizePolicy::PercentageH(percent) => {
                    remaining_height -= percent * content_area.width();
                }
                SizePolicy::PercentageV(percent) => {
                    remaining_height -= percent * content_area.height();
                }
                SizePolicy::Fraction(frac) => {
                    total_frac += frac;
                }
            }
        }

        let frac_height = remaining_height / total_frac;

        for item in self.children.iter_mut() {
            let height = match item.size().vertical {
                SizePolicy::Fixed(pixels) => pixels,
                SizePolicy::Percentage(percent) => percent * content_area.height(),
                SizePolicy::PercentageH(percent) => percent * content_area.width(),
                SizePolicy::PercentageV(percent) => percent * content_area.height(),
                SizePolicy::Fraction(frac) => frac * frac_height,
            };

            if let Some(mut child_context) =
                ctx.allocate_space((content_area.width(), height).into())
            {
                item.build(&mut child_context);
            }
        }

        /*let mut child = builder
            .new_child(content_area)
            .set_cursor_direction(CursorDirection::Vertical);
        for item in self.children().iter() {
            let height = match item.size().vertical {
                SizePolicy::Fixed(pixels) => pixels,
                SizePolicy::Percentage(percent) => percent * content_area.y,
                SizePolicy::PercentageH(percent) => percent * content_area.x,
                SizePolicy::PercentageV(percent) => percent * content_area.y,
                SizePolicy::Fraction(frac) => frac * frac_height,
            };
            child = child.widget(item.widget(), (content_area.x, height).into());
        }*/
        BuildResult::default()
    }

    fn children(&self) -> &[WidgetInstance] {
        &self.children
    }
}
