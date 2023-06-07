use rust_graphics::vec::Vec2;

use crate::error::{Error, Result};

use super::{
    widget::{SizePolicy2D, Widget},
    widget_builder::PushChild,
};

pub struct Container {
    size: Vec2,
    children: Vec<Box<dyn Widget>>,
}

impl Container {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            children: Vec::new(),
        }
    }

    pub fn add_child<T>(&mut self, child: T) -> Result<&dyn Widget>
    where
        T: Widget + 'static,
    {
        self.children.push(Box::new(child));
        Ok(self
            .children
            .last()
            .ok_or(Error::Generic(
                "Failed to push child to container for some reason!".into(),
            ))?
            .as_ref())
    }
}

impl Widget for Container {
    fn build(&self, push_child: &PushChild, size: SizePolicy2D) {
        for child in self.children.iter() {
            child.build(push_child, size.clone());
        }
    }
}
