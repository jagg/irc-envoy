extern crate irc_envoy;

use irc_envoy::irc;
use irc_envoy::display;
use std::{thread, time};

fn main() {
    println!("Starting IRC Envoy");
    let srv = "irc.mozilla.org".to_string();
    let _ = irc::IRCService::connect(irc::Config {
                                         server: srv,
                                         port: 6667,
                                     },
                                     display::cli::new());
    let hundred_secs = time::Duration::from_millis(100000);
    thread::sleep(hundred_secs);
}
