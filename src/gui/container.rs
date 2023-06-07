use crate::error::{Error, Result};

use super::{
    widget::{SizePolicy, SizePolicy2D, Widget},
    widget_builder::PushChild,
};

#[derive(Default)]
pub struct Container {
    children: Vec<Box<dyn Widget>>,
}

impl Container {
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
        push_child.push_child(size);
        {
            for child in self.children.iter() {
                push_child.widget(child.as_ref(), SizePolicy::Fill.into());
            }
        }
        push_child.pop_child();
    }
}
