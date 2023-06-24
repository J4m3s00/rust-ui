use crate::prelude::{SizePolicy, Widget};

use super::{build_context::BuildContext, build_results::BuildResult, size_policy::SizePolicy2D};

pub struct WidgetInstance {
    widget: Box<dyn Widget>,
    size: SizePolicy2D,
    // Stlying
    build_result: BuildResult,
}

impl<T> From<T> for WidgetInstance
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

impl WidgetInstance {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + 'static,
    {
        Self {
            widget: Box::new(widget),
            size: SizePolicy2D::default(),
            build_result: BuildResult::default(),
        }
    }

    pub fn build(&mut self, context: &mut BuildContext) {

        //context.allocate_space(self.size);
    }

    pub fn set_size(mut self, size: SizePolicy2D) -> Self {
        self.size = size;
        self
    }

    pub fn set_width(mut self, width: SizePolicy) -> Self {
        self.size.horizontal = width;
        self
    }

    pub fn set_height(mut self, height: SizePolicy) -> Self {
        self.size.vertical = height;
        self
    }

    pub fn widget(&self) -> &dyn Widget {
        &*self.widget
    }

    pub fn size(&self) -> SizePolicy2D {
        self.size
    }
}
