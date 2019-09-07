extern crate clap;
extern crate notify_rust;

use clap::{App, Arg};
use notify_rust::{Notification, Timeout};
use std::{thread, time};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_PERIOD: &str = "300";
const DEFAULT_TIMEOUT: &str = "10";

fn main() {
    let matches = App::new("reminders")
        .version(VERSION)
        .author("Antonio Gutierrez <chibby0ne@gmail.com>")
        .about("A periodical reminder program using desktop notifications")
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .help("Sets a custom message of the reminder. Default is \"Message\""),
        )
        .arg(
            Arg::with_name("period")
                .short("p")
                .long("period")
                .value_name("PERIOD")
                .help("Sets a custom period between reminders in seconds. Defaults to 300"),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .value_name("TIMEOUT")
                .help("before the notification dissapears. Default 10 seconds"),
        )
        .get_matches();
    let message = matches.value_of("message").unwrap_or("Hi there");
    let period = time::Duration::from_secs(
        matches
            .value_of("period")
            .unwrap_or(DEFAULT_PERIOD)
            .parse::<u64>()
            .unwrap(),
    );
    let timeout = Timeout::Milliseconds(
        1000 * matches
            .value_of("timeout")
            .unwrap_or(DEFAULT_TIMEOUT)
            .parse::<u32>()
            .unwrap(),
    );

    loop {
        Notification::new()
            .summary(message)
            .icon("dialog-information")
            .timeout(timeout)
            .show()
            .unwrap();
        thread::sleep(period);
    }
}
