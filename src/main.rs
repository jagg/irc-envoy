extern crate irc_envoy;

use irc_envoy::irc::{Config, Irc};
use std::{thread, time};

fn main() {
    println!("Starting IRC Envoy");
    let srv = "irc.mozilla.org".to_string();
    let irc = Irc::connect(Config {
        server: srv,
        port: 6667,
    });
    let hundred_secs = time::Duration::from_millis(100000);
    thread::sleep(hundred_secs);
}