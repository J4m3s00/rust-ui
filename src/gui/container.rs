use super::{
    widget::{self, SizePolicy, SizePolicy2D, Widget},
    widget_builder::PushChild,
};

pub struct ContainerItem {
    widget: Box<dyn Widget>,
    size: SizePolicy2D,
}

impl ContainerItem {
    pub fn set_size(&mut self, size: SizePolicy2D) -> &mut Self {
        self.size = size;
        self
    }
}

#[derive(Default)]
pub struct Container {
    children: Vec<ContainerItem>,
}

impl Container {
    pub fn add_child<T>(&mut self, child: T) -> &mut ContainerItem
    where
        T: Widget + 'static,
    {
        self.children.push(ContainerItem {
            widget: Box::new(child),
            size: SizePolicy::Fill.into(),
        });
        self.children.last_mut().expect("Failed to add child")
    }
}

impl Widget for Container {
    fn build(&self, push_child: &PushChild, size: SizePolicy2D) {
        push_child.push_child(size);
        {
            for item in self.children.iter() {
                push_child.widget(item.widget.as_ref(), item.size);
            }
        }
        push_child.pop_child();
    }
}
