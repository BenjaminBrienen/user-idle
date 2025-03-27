use core::{mem::size_of, ptr::addr_of_mut, time::Duration};

use windows_sys::Win32::{
    System::SystemInformation::GetTickCount,
    UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
};

use crate::error::Error;

// Based on https://bitbucket.org/pidgin/main/src/8066acc5ed9306c5a53da8f66f50fb5cf38782c7/pidgin/win32/gtkwin32dep.c#lines-597

/// Get the idle time of a user.
/// 
/// # Panics
/// 
/// Panics if a system call fails or if time flows backwards.
#[inline]
pub fn get_idle_time() -> Result<Duration, Error> {
    let now = unsafe { GetTickCount() };

    #[expect(clippy::as_conversions, reason = "manually validated")]
    const CB_SIZE: u32 = size_of::<LASTINPUTINFO>() as u32;

    let mut last_input_info = LASTINPUTINFO {
        cbSize: CB_SIZE,
        dwTime: 0,
    };

    if unsafe { GetLastInputInfo(addr_of_mut!(last_input_info)) } == 0 {
        Err(Error::new("GetLastInputInfo failed"))
    } else {
        Ok(Duration::from_millis(u64::from(
            now.checked_sub(last_input_info.dwTime).expect("now < last_input_info.dwTime"),
        )))
    }
}
