use display::IRCDisplay;
use super::super::data;
use std::io;

pub struct SimpleDisplay {
    
}

impl IRCDisplay for SimpleDisplay {
    fn show(&self, input: data::Msg) -> Result<(), io::Error> {
        match input {
            data::Msg::PrivContent { origin: o, destination: d, msg } => {
                println!("[{} -> {}] {}", o, d, msg)
            }
            data::Msg::ChanContent { origin: o, destination: d, msg } => {
                println!("[{} -> #{}] {}", o, d, msg)
            }
            data::Msg::Other(msg) => println!("... {}", msg),
            _ => println!("This should't happen!"),

        }
        Ok(())
    }
}

pub fn new() -> SimpleDisplay {
    SimpleDisplay {}
}