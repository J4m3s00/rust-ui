use super::{
    widget::{SizePolicy, SizePolicy2D, Widget},
    widget_builder::WidgetBuilder,
};

pub trait ContainerItem {
    // Sets the size of the item.
    fn set_size(self, size: SizePolicy2D) -> Self;

    // Returns the widget of the item.
    fn widget(&self) -> &dyn Widget;

    // Returns the size of the item.
    fn size(&self) -> SizePolicy2D;
}

pub trait Container {
    type Item: ContainerItem;
    fn children(&self) -> &[Self::Item];
}

impl<T> Widget for T
where
    T: Container,
{
    fn build(&self, build: &mut WidgetBuilder, size: SizePolicy2D) {
        let mut child = build.child(size);
        for item in self.children().iter() {
            child = child.widget(item.widget(), item.size());
        }
    }
}

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
