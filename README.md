# reminders

A periodical reminder program using desktop notifications

## Requirements

* rust
* libnotify
* A desktop notification server

##  Installation

From crates.io:

```bash
cargo install reminders
```

From source, build:

```
cargo build --release
```

Then copy the binary `target/release/reminders` to a place available in your `$PATH` variable.

## Usage

Before the first use you should edit the `ExecStart` line in the
[reminders.service](reminders.service) file provided in this repository.

```
ExecStart=reminders -m MESSAGE -p 300
```

Change the `MESSAGE` with whatever message you want your reminder to show, as
well the `300` to the reminder period you want, given in seconds.

Afterwards you should install/start the `reminders.service` systemd unit,
so that the process actually keeps running in the background.

## TODO

* Configuration file
* Add AUR package
* Handle `systemctl stop`.
* Expose more notification options

## Is it any good?

[yes](https://news.ycombinator.com/item?id=3067434)
