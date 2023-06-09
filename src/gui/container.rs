use super::{
    widget::{SizePolicy2D, Widget},
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
