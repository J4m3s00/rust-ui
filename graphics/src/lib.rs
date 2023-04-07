use bindings::InitApp;

use crate::bindings::c_start_application;

pub mod app;
mod bindings;
pub mod version;

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
