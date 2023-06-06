#[derive(Debug)]
pub enum SizePolicy {
    Fixed(f32),
    Percentage(f32),
    Fill,
}

pub type WidgetSize = (SizePolicy, SizePolicy);

#[derive(Debug)]
pub struct Widget {
    pub size: WidgetSize,
}
