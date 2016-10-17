use display::IRCDisplay;
use super::super::parser;
use std::io;

pub struct SimpleDisplay {
    
}

impl IRCDisplay for SimpleDisplay {
    fn show(&self, input: parser::Msg) -> Result<(), io::Error> {
        match input {
            parser::Msg::PrivContent { origin: o, destination: d, msg } => {
                println!("[{} -> {}] {}", o, d, msg)
            }
            parser::Msg::ChanContent { origin: o, destination: d, msg } => {
                println!("[{} -> #{}] {}", o, d, msg)
            }
            parser::Msg::Other(msg) => println!("... {}", msg),
            _ => println!("This should't happen!"),

        }
        Ok(())
    }
}

pub fn new() -> SimpleDisplay {
    SimpleDisplay {}
}