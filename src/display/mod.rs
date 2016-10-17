
use super::data;
use std::io;
use std::marker;

pub mod cli;

pub trait IRCDisplay: marker::Send {
    fn show(&self, input: data::Msg) -> Result<(), io::Error>;
}