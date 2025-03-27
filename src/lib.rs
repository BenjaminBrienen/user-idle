//! Get the idle time of a user.
//! The time returned is the time since the last user input event.
//!
//! Example:
//! ```
//! use user_idle_time::get_idle_time;
//! let idle = get_idle_time().unwrap();
//! let idle_seconds = idle.as_secs();
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
    const DURATION: Duration = Duration::from_secs(TEST_SECS);

    #[test]
    // If this test fails, you probably moved your mouse or something while the test was running.
    fn main() {
        let idle_before = get_idle_time().expect("Failed to get idle time 1");
        sleep(DURATION);
        let idle_after = get_idle_time().expect("Failed to get idle time 2");
        assert_eq!(idle_after.checked_sub(idle_before), Some(DURATION));
    }
}
