use super::widget::{SizePolicy2D, Widget};

pub struct ContainerItem {
    widget: Box<dyn Widget>,
    size: SizePolicy2D,
}

impl<T> From<T> for ContainerItem
where
    T: Widget + 'static,
{
    fn from(widget: T) -> Self
    where
        T: Widget + 'static,
    {
        Self::new(widget)
    }
}

impl ContainerItem {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + 'static,
    {
        Self {
            widget: Box::new(widget),
            size: SizePolicy2D::default(),
        }
    }

    pub fn set_size(mut self, size: SizePolicy2D) -> Self {
        self.size = size;
        self
    }

    pub fn widget(&self) -> &dyn Widget {
        &*self.widget
    }

    pub fn size(&self) -> SizePolicy2D {
        self.size
    }
}
