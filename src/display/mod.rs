
use super::parser;
use std::io;
use std::marker;

pub mod cli;

pub trait IRCDisplay: marker::Send {
    fn show(&self, input: parser::Msg) -> Result<(), io::Error>;
}