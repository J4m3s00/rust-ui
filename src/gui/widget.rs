use super::widget_builder::WidgetBuilder;

#[derive(Debug, Clone, Copy)]
pub enum SizePolicy {
    Fixed(f32),
    Percentage(f32),
    Fill,
}

pub type SizePolicy2D = (SizePolicy, SizePolicy);

pub trait Widget {
    fn build(&self, build: &mut WidgetBuilder, size: SizePolicy2D);
}
