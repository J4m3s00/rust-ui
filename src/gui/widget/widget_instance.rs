use std::fmt::Debug;

use rust_graphics::{cursor::SystemCursor, rect::Rect};

use crate::{
    gui::events::{keyboard::KeyboardEvent, mouse::MouseEvent},
    prelude::{AppInterface, Margin, SizePolicy, State, Style, Widget},
};

use super::{
    builder::{build_context::BuildContext, build_results::BuildResult},
    iterator::WidgetIter,
    size_policy::SizePolicy2D,
    state::observable::Observer,
    widget::WidgetMouseState,
};

pub struct WidgetInstance {
    type_name: &'static str,
    id: usize,

    widget: Box<dyn Widget>,
    visible: Observer<bool>,
    mouse_state: State<WidgetMouseState>,
    focused: State<bool>,
    // Stlying
    size: SizePolicy2D,
    style: Style,
    custom_cursor: Option<SystemCursor>,

    // Build Results
    build_result: BuildResult,
    build_rect: Rect,
    receive_input: bool,
    child_input_ids: Vec<usize>,
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

fn get_id() -> usize {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

impl WidgetInstance {
    pub fn new<T>(widget: T) -> Self
    where
        T: Widget + 'static,
    {
        Self {
            widget: Box::new(widget),
            id: get_id(),
            visible: true.into(),
            mouse_state: WidgetMouseState::Normal.into(),
            focused: false.into(),
            size: SizePolicy2D::default(),
            style: Style::default(),
            custom_cursor: None,
            build_result: BuildResult::default(),
            build_rect: Rect::default(),
            child_input_ids: Vec::new(),
            receive_input: false,
            type_name: std::any::type_name::<T>(),
        }
    }

    pub fn accept_input(mut self) -> Self {
        self.receive_input = true;
        self
    }

    pub fn custom_cursor(mut self, cursor: SystemCursor) -> Self {
        self.custom_cursor = Some(cursor);
        self
    }

    /// When tab press focuses the widget
    pub fn on_focus(&self) {
        self.focused.set(true);
        self.widget.on_gain_focus();
    }

    pub fn on_lose_focus(&self) {
        self.focused.set(false);
        self.widget.on_lose_focus();
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn does_accept_input(&self) -> bool {
        self.receive_input
    }

    pub fn build(&mut self, context: &mut BuildContext) {
        //context.allocate_space(self.size);
        self.build_rect = *context.get_content_rect();
        context.set_style(self.style.clone());
        self.build_result =
            self.widget
                .build(context, self.mouse_state.observe(), self.focused.observe());
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

    pub fn build_result(&self) -> (&BuildResult, &Rect, &Vec<usize>) {
        (&self.build_result, &self.build_rect, &self.child_input_ids)
    }

    pub fn type_name(&self) -> &'static str {
        self.type_name
    }

    pub fn handle_mouse_event(&self, event: &MouseEvent, app_interface: AppInterface) {
        if event.mouse_entered {
            if let Some(cursor) = &self.custom_cursor {
                app_interface.change_cursor(cursor.clone());
            }
            self.widget.on_mouse_enter(event, app_interface.clone());
        }
        if event.mouse_exited {
            if let Some(_) = &self.custom_cursor {
                app_interface.change_cursor(SystemCursor::Arrow);
            }
            self.widget.on_mouse_leave(event, app_interface.clone());
        }
        if event.button_down.is_some() {
            self.widget.on_mouse_down(event, app_interface.clone());
        }
        if event.button_up.is_some() {
            self.widget.on_mouse_up(event, app_interface.clone());
        }
        if event.delta != (0.0, 0.0).into() {
            self.widget.on_mouse_move(event, app_interface);
        }

        match self.mouse_state.get() {
            WidgetMouseState::Normal => {
                if event.inside {
                    self.mouse_state.set(WidgetMouseState::Hovered);
                }
            }
            WidgetMouseState::Hovered => {
                if event.mouse_exited {
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

    pub fn handle_keyboard_event(&self, event: &KeyboardEvent, app_interface: AppInterface) {
        match event {
            KeyboardEvent::KeyDown(kc, mods) => self.widget.on_key_down(*kc, *mods, app_interface),
            KeyboardEvent::KeyUp(kc, mods) => self.widget.on_key_up(*kc, *mods, app_interface),
            KeyboardEvent::Text(text) => self.widget.on_text_input(text.clone(), app_interface),
        }
    }
}
