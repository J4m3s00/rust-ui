use super::container::{Container, ContainerWidgetItem};

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
