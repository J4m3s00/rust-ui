use crate::style::Style;

pub trait Widget {
    fn get_style(&self) -> &Style;
    fn get_style_mut(&mut self) -> &mut Style;
}
