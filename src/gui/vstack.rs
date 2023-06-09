use super::{
    container::{Container, ContainerItem},
    widget::{SizePolicy, SizePolicy2D, Widget},
};

pub struct ContainerWidgetItem {
    widget: Box<dyn Widget>,
    size: SizePolicy2D,
}

impl ContainerWidgetItem {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + 'static,
    {
        Self {
            widget: Box::new(widget),
            size: SizePolicy::Percentage(1.).into(),
        }
    }
}

impl ContainerItem for ContainerWidgetItem {
    fn set_size(mut self, size: SizePolicy2D) -> Self {
        self.size = size;
        self
    }

    fn widget(&self) -> &dyn Widget {
        &*self.widget
    }

    fn size(&self) -> SizePolicy2D {
        self.size
    }
}

pub struct VStack {
    children: Vec<ContainerWidgetItem>,
}

impl VStack {
    pub fn new(children: Vec<ContainerWidgetItem>) -> Self {
        Self { children }
    }
}

impl Container for VStack {
    type Item = ContainerWidgetItem;

    fn children(&self) -> &[ContainerWidgetItem] {
        &self.children
    }
}
