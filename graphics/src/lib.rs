#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
use bindings::{c_clean_up_editor, c_post_update_application, c_pre_update_application, InitApp};

use crate::bindings::{c_start_application, AppEventType_AppEventType_Quit};

pub mod app;
mod bindings;
pub mod color;
pub mod rect;
pub mod style;
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
