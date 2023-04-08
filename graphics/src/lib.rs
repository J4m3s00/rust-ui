#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
use bindings::{
    c_clean_up_editor, c_draw_circle, c_draw_line, c_draw_rect, c_draw_text,
    c_post_update_application, c_pre_update_application, InitApp,
};
use draw_command::DrawCommand;

use crate::bindings::{c_start_application, AppEventType_AppEventType_Quit};

pub mod app;
mod bindings;
pub mod circle;
pub mod color;
pub mod draw_command;
pub mod rect;
pub mod style;
pub mod text;
pub mod vec;
pub mod version;
pub mod widget;

pub enum AppEvent {
    None,
    Quit,
}

pub fn start_editor(title: impl Into<String>) -> i32 {
    let c_msg = match std::ffi::CString::new(title.into().as_str()) {
        Ok(s) => s,
        Err(_e) => return 0,
    };
    let init_app = InitApp {
        title: c_msg.as_ptr(),
    };
    unsafe { c_start_application(&init_app) }
}

pub fn update_editor<T>(mut render_func: T) -> AppEvent
where
    T: FnMut(),
{
    let event = unsafe { *c_pre_update_application() };
    render_func();
    unsafe { c_post_update_application() };
    match event.type_ {
        AppEventType_AppEventType_Quit => AppEvent::Quit,
        _ => AppEvent::None,
    }
}

pub fn quit_editor() {
    unsafe { c_clean_up_editor() };
}

pub fn run_draw_command(command: DrawCommand) {
    match command {
        DrawCommand::Rect(rect) => unsafe {
            c_draw_rect(
                rect.left,
                rect.bottom,
                rect.right - rect.left,
                rect.top - rect.bottom,
            )
        },
        DrawCommand::Circle(circle) => unsafe {
            c_draw_circle(circle.center.x, circle.center.y, circle.radius);
        },
        DrawCommand::Line { x1, y1, x2, y2 } => unsafe {
            c_draw_line(x1, y1, x2, y2);
        },
        DrawCommand::Text(text) => unsafe {
            let c_msg = std::ffi::CString::new(text.text.as_str())
                .map_err(|_| println!("Error: Could not convert text to CString"))
                .unwrap_or_default();
            c_draw_text(text.position.x, text.position.y, c_msg.as_ptr());
        },
    }
}
