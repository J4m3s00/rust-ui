use std::fmt::Debug;

use rust_graphics::rect::Rect;

use crate::prelude::{SizePolicy, Widget};

use super::{
    build_context::BuildContext, build_results::BuildResult, iterator::WidgetIter,
    size_policy::SizePolicy2D,
};

pub struct WidgetInstance {
    type_name: &'static str,

    widget: Box<dyn Widget>,
    size: SizePolicy2D,
    // Stlying
    build_result: BuildResult,
    build_rect: Rect,
}

impl Debug for WidgetInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetInstance")
            .field("size", &self.size)
            .field("build_result", &self.build_result)
            .field("build_rect", &self.build_rect)
            .finish()
    }
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
            build_rect: Rect::default(),
            type_name: std::any::type_name::<T>(),
        }
    }

    pub fn build(&mut self, context: &mut BuildContext) {
        //context.allocate_space(self.size);
        self.build_rect = context.get_content_rect().clone();
        self.build_result = self.widget.build(context);
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

    pub fn iter(&self) -> WidgetIter {
        WidgetIter::new(self)
    }

    pub fn build_result(&self) -> (&BuildResult, &Rect) {
        (&self.build_result, &self.build_rect)
    }

    pub fn type_name(&self) -> &'static str {
        self.type_name
    }
}
