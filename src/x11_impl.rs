//! Implementation of [`get_idle_time`] for X11.

use std::{ffi::c_void, os::raw::c_char, ptr::null, time::Duration};

use x11::{
    xlib::{XCloseDisplay, XDefaultScreen, XFree, XOpenDisplay, XRootWindow},
    xss::{XScreenSaverAllocInfo, XScreenSaverQueryInfo},
};

use crate::error::Error;

// Mostly taken from https://stackoverflow.com/questions/222606/detecting-keyboard-mouse-activity-in-linux

/// Get the idle time of a user.
///
/// # Panics
///
/// Panics if a system call fails or if time flows backwards.
#[inline]
pub fn get_idle_time() -> Result<Duration, Error> {
    unsafe {
        let info = XScreenSaverAllocInfo();
        let display = XOpenDisplay(null::<c_char>());
        if (display.is_null()) {
            return Err(Error::new("Failed to open display"));
        }
        let screen = XDefaultScreen(display);
        let root_window = XRootWindow(display, screen);
        let status = XScreenSaverQueryInfo(display, root_window, info);
        let time = (*info).idle;

        XFree(info.cast::<c_void>());
        XCloseDisplay(display);

        if status == 1 {
            Ok(Duration::from_millis(time))
        } else {
            Err(Error::new("Status not OK"))
        }
    }
}
