extern "C" {
    fn c_hello_world() -> i32;
}

pub fn hello_world() -> i32 {
    unsafe { c_hello_world() }
}
