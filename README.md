# `user-idle-time`

Get the idle time of a user

| OS              | Supported |
| --------------- | --------- |
| Linux (x11)     | ✔️         |
| Linux (dbus)    | ✔️*        |
| Linux (wayland) | ❌         |
| Windows         | ✔️         |
| MacOS           | ❌         |

> [!NOTE]
> DBus returns the time the session has been locked, not the time since the last user input event.
>
> By default, x11 is used on Linux. DBus can be enabled in `Cargo.toml` by disabling default-features and enabling `dbus`.

## Example

```rust
use user_idle_time::get_idle_time;
let idle = get_idle_time().unwrap();
let idle_seconds = idle.as_secs();
```

Check the [documentation](https://docs.rs/user-idle-time/latest/user-idle-time/) for more methods.
