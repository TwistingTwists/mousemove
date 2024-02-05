#[cfg(target_os = "windows")]
pub fn get_screen_size() -> (i32, i32) {
    use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) as i32 };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) as i32 };
    (width, height)
}

#[cfg(target_os = "macos")]
pub fn get_screen_size() -> (i32, i32) {
    use core_graphics::display::CGDisplay;
    let main_display = unsafe { CGDisplay::main().unwrap() };
    let width = main_display.bounds().size.width as i32;
    let height = main_display.bounds().size.height as i32;
    (width, height)
}

#[cfg(target_os = "linux")]
pub fn get_screen_size() -> (i32, i32) {
    use x11::xlib::{Display, XCloseDisplay, XFree, XOpenDisplay};
    use x11::xrandr::{
        XRRFreeScreenInfo, XRRFreeScreenResources, XRRGetScreenInfo, XRRGetScreenResources,
    };

    let dpy = unsafe { XOpenDisplay(std::ptr::null()) };
    let res = unsafe { XRRGetScreenResources(dpy, std::ptr::null_mut()) };
    let primary_output = unsafe { (*res).primaryOutput };
    let info = unsafe { XRRGetScreenInfo(dpy, res, primary_output) };
    let width = unsafe { (*info).width };
    let height = unsafe { (*info).height };

    unsafe {
        XRRFreeScreenInfo(info);
        XRRFreeScreenResources(res);
        XCloseDisplay(dpy);
    }

    (width as i32, height as i32)
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn get_screen_size() -> (i32, i32) {
    (1920, 1080) // Default screen size for other platforms
}
