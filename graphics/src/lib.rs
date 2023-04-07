use bindings::c_hello_world;

use crate::bindings::c_start_application;

mod bindings;

pub fn hello_world() -> i32 {
    unsafe { c_hello_world() }
}

pub fn start_editor() -> i32 {
    unsafe { c_start_application() }
}
