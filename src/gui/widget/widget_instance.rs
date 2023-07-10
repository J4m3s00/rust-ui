use std::fmt::Debug;

use rust_graphics::rect::Rect;

use crate::prelude::{AppInterface, Margin, SizePolicy, State, Style, Widget};

use super::{
    builder::{build_context::BuildContext, build_results::BuildResult},
    iterator::WidgetIter,
    size_policy::SizePolicy2D,
    state::observable::Observer,
    widget::{MouseEvent, WidgetMouseState},
};

pub struct WidgetInstance {
    type_name: &'static str,

    widget: Box<dyn Widget>,
    visible: Observer<bool>,
    mouse_state: State<WidgetMouseState>,
    // Stlying
    size: SizePolicy2D,
    style: Style,

    // Build Results
    build_result: BuildResult,
    build_rect: Rect,
}

impl Debug for WidgetInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetInstance")
            .field("size", &self.size)
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
            visible: true.into(),
            mouse_state: WidgetMouseState::Normal.into(),
            size: SizePolicy2D::default(),
            style: Style::default(),
            build_result: BuildResult::default(),
            build_rect: Rect::default(),
            type_name: std::any::type_name::<T>(),
        }
    }

    pub fn build(&mut self, context: &mut BuildContext) {
        //context.allocate_space(self.size);
        self.build_rect = context.get_content_rect().clone();
        context.set_style(self.style.clone());
        self.build_result = self.widget.build(context, &self.mouse_state);
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

    pub fn set_margin(mut self, margin: Margin) -> Self {
        self.style.margin = margin;
        self
    }

    pub fn set_padding(mut self, padding: Margin) -> Self {
        self.style.padding = padding;
        self
    }

    pub fn set_visible(mut self, visible: impl Into<Observer<bool>>) -> Self {
        self.visible = visible.into();
        self
    }

    pub fn widget(&self) -> &dyn Widget {
        &*self.widget
    }

    pub fn size(&self) -> SizePolicy2D {
        self.size
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn visible(&self) -> bool {
        self.visible.get().unwrap()
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

    pub fn handle_mouse_event(&self, event: &MouseEvent, app_interface: AppInterface) {
        if event.mouse_entered {
            self.widget
                .on_mouse_enter(event.clone(), app_interface.clone());
        }
        if event.mouse_exited {
            self.widget
                .on_mouse_leave(event.clone(), app_interface.clone());
        }
        if event.button_down.is_some() {
            self.widget
                .on_mouse_down(event.clone(), app_interface.clone());
        }
        if event.button_up.is_some() {
            self.widget
                .on_mouse_up(event.clone(), app_interface.clone());
        }
        if event.delta != (0.0, 0.0).into() {
            self.widget
                .on_mouse_move(event.clone(), app_interface.clone());
        }

        match self.mouse_state.get() {
            WidgetMouseState::Normal => {
                if event.inside {
                    self.mouse_state.set(WidgetMouseState::Hovered);
                }
            }
            WidgetMouseState::Hovered => {
                if !event.inside {
                    self.mouse_state.set(WidgetMouseState::Normal);
                } else if let Some(1) = event.button_down {
                    self.mouse_state.set(WidgetMouseState::Pressed);
                }
            }
            WidgetMouseState::Pressed => {
                if !event.inside {
                    self.mouse_state.set(WidgetMouseState::Normal);
                } else if let Some(1) = event.button_up {
                    self.mouse_state.set(WidgetMouseState::Hovered);
                }
            }
        }
    }
}
