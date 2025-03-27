//! Get the idle time of a user.
//! The time returned is the time since the last user input event.
//!
//! Example:
//! ```
//! use user_idle_time::UserIdle;
//! let idle = UserIdle::get_time().unwrap();
//! let idle_seconds = idle.as_seconds();
//! let idle_minutes = idle.as_minutes();
//! ```

mod error;

pub use error::Error;

#[cfg(all(target_os = "linux", not(feature = "dbus")))]
#[path = "x11_impl.rs"]
mod idle;

#[cfg(all(target_os = "linux", feature = "dbus"))]
#[path = "dbus_impl.rs"]
mod idle;

#[cfg(target_os = "windows")]
#[path = "windows_impl.rs"]
mod idle;

#[cfg(target_os = "macos")]
#[path = "macos_impl.rs"]
mod idle;

pub use idle::get_idle_time;

#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use super::get_idle_time;

    const TEST_SECS: u64 = 10;

    #[test]
    fn main() {
        sleep(Duration::from_secs(TEST_SECS));
        let idle = get_idle_time().unwrap();
        assert_eq!(idle.as_secs(), TEST_SECS);
    }
}
