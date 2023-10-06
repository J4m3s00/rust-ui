use rust_graphics::{events::app_events::AppEvent, vec::Vec2};

use crate::prelude::WidgetInstance;

#[derive(Default, Clone)]
pub struct MouseEvent {
    pub relative_pos: Vec2,
    pub absolute_pos: Vec2,
    pub delta: Vec2,

    pub button_down: Option<i32>,
    pub button_up: Option<i32>,
    pub mouse_entered: bool,
    pub mouse_exited: bool,
    pub inside: bool,
}

impl MouseEvent {
    pub fn from_app_event(
        event: &AppEvent,
        widget: &WidgetInstance,
        last_mouse_pos: Vec2,
    ) -> Option<Self> {
        let (_, build_rect, _) = widget.build_result();
        match event {
            AppEvent::MouseMove { x, y } => {
                let absolute_pos = Vec2::new(*x as f32, *y as f32);
                let relative_pos = absolute_pos - build_rect.position();
                let delta = absolute_pos - last_mouse_pos;
                let mouse_entered =
                    build_rect.contains(absolute_pos) && !build_rect.contains(last_mouse_pos);
                let mouse_exited =
                    !build_rect.contains(absolute_pos) && build_rect.contains(last_mouse_pos);

                Some(MouseEvent {
                    relative_pos,
                    absolute_pos,
                    delta,
                    inside: build_rect.contains(absolute_pos),
                    mouse_entered,
                    mouse_exited,
                    button_down: None,
                    button_up: None,
                })
            }
            AppEvent::MouseDown { key, x, y } => {
                let absolute_pos = Vec2::new(*x as f32, *y as f32);
                let relative_pos = absolute_pos - build_rect.position();
                let delta = absolute_pos - last_mouse_pos;
                let mouse_entered =
                    build_rect.contains(absolute_pos) && !build_rect.contains(last_mouse_pos);
                let mouse_exited =
                    !build_rect.contains(absolute_pos) && build_rect.contains(last_mouse_pos);

                Some(MouseEvent {
                    relative_pos,
                    absolute_pos,
                    delta,
                    inside: build_rect.contains(absolute_pos),
                    mouse_entered,
                    mouse_exited,
                    button_down: Some(*key),
                    button_up: None,
                })
            }
            AppEvent::MouseUp { key, x, y } => {
                let absolute_pos = Vec2::new(*x as f32, *y as f32);
                let relative_pos = absolute_pos - build_rect.position();
                let delta = absolute_pos - last_mouse_pos;
                let mouse_entered =
                    build_rect.contains(absolute_pos) && !build_rect.contains(last_mouse_pos);
                let mouse_exited =
                    !build_rect.contains(absolute_pos) && build_rect.contains(last_mouse_pos);

                Some(MouseEvent {
                    relative_pos,
                    absolute_pos,
                    delta,
                    inside: build_rect.contains(absolute_pos),
                    mouse_entered,
                    mouse_exited,
                    button_down: None,
                    button_up: Some(*key),
                })
            }
            _ => None,
        }
    }
}
