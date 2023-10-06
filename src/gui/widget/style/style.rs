use crate::prelude::Margin;

use super::space::Padding;

#[derive(Clone, Debug)]
pub struct Style {
    pub margin: Margin,
    pub padding: Padding,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            margin: Margin::new_all(2.),
            padding: Padding::zero(),
        }
    }
}
