use super::widget_builder::WidgetBuilder;

#[derive(Debug, Clone, Copy)]
pub enum SizePolicy {
    Fixed(f32),
    Percentage(f32),
    Fill,
}

#[derive(Debug, Clone, Copy)]
pub struct SizePolicy2D {
    pub horizontal: SizePolicy,
    pub vertical: SizePolicy,
}

impl From<SizePolicy> for SizePolicy2D {
    fn from(policy: SizePolicy) -> Self {
        Self {
            horizontal: policy,
            vertical: policy,
        }
    }
}

impl From<(SizePolicy, SizePolicy)> for SizePolicy2D {
    fn from((horizontal, vertical): (SizePolicy, SizePolicy)) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

pub trait Widget {
    fn build(&self, build: &mut WidgetBuilder, size: SizePolicy2D);
}
