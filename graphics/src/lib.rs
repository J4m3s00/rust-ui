use bindings::c_hello_world;

use crate::bindings::c_start_application;

pub mod app;
mod bindings;
pub mod version;

pub fn hello_world() -> i32 {
    unsafe { c_hello_world() }
}

pub fn start_editor() -> i32 {
    unsafe { c_start_application() }
}
