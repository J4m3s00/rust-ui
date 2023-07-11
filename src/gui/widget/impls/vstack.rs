use crate::{
    gui::widget::{
        builder::{
            build_context::{BuildContext, CursorDirection},
            build_results::BuildResult,
        },
        rendering::drawable::rectangle::DrawRect,
        size_policy::SizePolicy,
        widget::{ToInstance, WidgetMouseState},
    },
    prelude::{ColorId, State, Widget, WidgetInstance},
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
    fn build(&mut self, ctx: &mut BuildContext, _: &State<WidgetMouseState>) -> BuildResult {
        // Update context cursor direction for the children
        ctx.set_cursor_direction(CursorDirection::Vertical);
        let content_area = *ctx.get_content_rect();
        let mut remaining_height = content_area.height();
        let mut total_frac = 0.;
        for item in self.children().iter() {
            match item.size().vertical {
                SizePolicy::Fixed(pixels) => {
                    remaining_height -= pixels;
                }
                SizePolicy::Percent(x) | SizePolicy::PercentageV(x) => {
                    remaining_height -= x * content_area.width();
                }
                SizePolicy::PercentageH(p) => {
                    remaining_height -= p * content_area.height();
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
                SizePolicy::Percent(x) | SizePolicy::PercentageH(x) => x * content_area.width(),
                SizePolicy::PercentageV(p) => p * content_area.height(),
                SizePolicy::Fraction(frac) => frac * frac_height,
            };

            if let Some(mut child_context) = ctx.allocate_space((content_area.width(), height)) {
                item.build(&mut child_context);
            }
        }
        let mut res = BuildResult::default();
        res.draw_rect(DrawRect::fill(Some(ColorId::Background)));
        res
    }

    fn children(&self) -> &[WidgetInstance] {
        &self.children
    }
}
