/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InitApp {
    pub title: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_InitApp() {
    const UNINIT: ::std::mem::MaybeUninit<InitApp> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<InitApp>(),
        8usize,
        concat!("Size of: ", stringify!(InitApp))
    );
    assert_eq!(
        ::std::mem::align_of::<InitApp>(),
        8usize,
        concat!("Alignment of ", stringify!(InitApp))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).title) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(InitApp),
            "::",
            stringify!(title)
        )
    );
}
pub const AppEventType_AppEventType_None: AppEventType = 0;
pub const AppEventType_AppEventType_Quit: AppEventType = 1;
pub const AppEventType_AppEventType_KeyDown: AppEventType = 2;
pub const AppEventType_AppEventType_KeyUp: AppEventType = 3;
pub const AppEventType_AppEventType_MouseDown: AppEventType = 4;
pub const AppEventType_AppEventType_MouseUp: AppEventType = 5;
pub const AppEventType_AppEventType_MouseMove: AppEventType = 6;
pub const AppEventType_AppEventType_MouseWheel: AppEventType = 7;
pub const AppEventType_AppEventType_WindowResize: AppEventType = 8;
pub type AppEventType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppEvent {
    pub type_: AppEventType,
    pub key: ::std::os::raw::c_int,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AppEvent() {
    const UNINIT: ::std::mem::MaybeUninit<AppEvent> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AppEvent>(),
        16usize,
        concat!("Size of: ", stringify!(AppEvent))
    );
    assert_eq!(
        ::std::mem::align_of::<AppEvent>(),
        4usize,
        concat!("Alignment of ", stringify!(AppEvent))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AppEvent),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AppEvent),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(AppEvent),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(AppEvent),
            "::",
            stringify!(y)
        )
    );
}
extern "C" {
    pub fn c_start_application(initApp: *const InitApp) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn c_pre_update_application() -> *mut AppEvent;
}
extern "C" {
    pub fn c_post_update_application();
}
extern "C" {
    pub fn c_clean_up_editor();
}
extern "C" {
    pub fn c_draw_rect(x: f32, y: f32, width: f32, height: f32);
}
extern "C" {
    pub fn c_draw_circle(x: f32, y: f32, radius: f32);
}
extern "C" {
    pub fn c_draw_line(x1: f32, y1: f32, x2: f32, y2: f32);
}
extern "C" {
    pub fn c_draw_text(x: f32, y: f32, text: *const ::std::os::raw::c_char);
}
