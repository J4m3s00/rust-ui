use super::{
    widget::{SizePolicy, SizePolicy2D, Widget},
    widget_builder::WidgetBuilder,
};

pub struct ContainerItem {
    widget: Box<dyn Widget>,
    size: SizePolicy2D,
}

impl ContainerItem {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + 'static,
    {
        Self {
            widget: Box::new(widget),
            size: SizePolicy::Percentage(1.).into(),
        }
    }

    pub fn set_size(mut self, size: SizePolicy2D) -> Self {
        self.size = size;
        self
    }
}

#[derive(Default)]
pub struct Container {
    pub children: Vec<ContainerItem>,
}

impl Container {
    pub fn add_child<T>(&mut self, child: T) -> &mut ContainerItem
    where
        T: Widget + 'static,
    {
        self.children.push(ContainerItem {
            widget: Box::new(child),
            size: SizePolicy::Percentage(1.0).into(),
        });
        self.children.last_mut().expect("Failed to add child")
    }
}

impl Widget for Container {
    fn build(&self, push_child: &mut WidgetBuilder, size: SizePolicy2D) {
        let mut composer = push_child.child(size);

        for item in self.children.iter() {
            composer = composer.widget(item.widget.as_ref(), item.size);
        }
    }
}
