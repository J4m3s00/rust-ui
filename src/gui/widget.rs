use std::ops::Deref;

use rust_graphics::vec::Vec2;

use super::{container::ContainerItem, widget_builder::WidgetBuilder};

#[derive(Debug, Clone, Copy)]
pub enum SizePolicy {
    Fixed(f32),
    Percentage(f32),
    Fraction(f32),
}

impl Default for SizePolicy {
    fn default() -> Self {
        Self::Fraction(1.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
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
    fn build(&self, builder: &mut WidgetBuilder, size: Vec2);
    fn calc_min_size(&self, size: SizePolicy2D) -> Vec2;
}

pub trait ToItem {
    fn into_item(self) -> ContainerItem;
}

impl<T> ToItem for T
where
    T: Widget + 'static,
{
    fn into_item(self) -> ContainerItem {
        ContainerItem::new(self)
    }
}
