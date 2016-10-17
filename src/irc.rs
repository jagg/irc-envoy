use std::time;
use std::net;
use std::io;
use std::io::{BufReader, BufRead, BufWriter};
use std::io::prelude::Write;
use std::thread;

use super::parser;
use super::display;

pub struct Config {
    pub server: String,
    pub port: u16,
}

pub struct IRCService {
    tcp_writer: BufWriter<net::TcpStream>,
}

impl IRCService {
    pub fn connect<T: display::IRCDisplay + 'static>(config: Config,
                                                     display: T)
                                                     -> io::Result<IRCService> {
        let stream = try!(net::TcpStream::connect((config.server.as_str(), config.port)));
        let buf_writer = BufWriter::new(stream.try_clone().unwrap());
        let reader = stream.try_clone().unwrap();
        let mut reader = BufReader::new(reader);
        // let back_writer = BufWriter::new(stream.try_clone().unwrap());
        let mut service = IRCService { tcp_writer: buf_writer };

        thread::spawn(move || {
            let mut buffer = String::new();
            loop {
                reader.read_line(&mut buffer).unwrap();
                let msg = parser::parse(buffer.as_bytes());
                let _ = display.show(msg);
            }
        });

        try!(service.login("irc-envoy"));
        thread::sleep(time::Duration::from_millis(2000));
        try!(service.join("irc-test"));

        Ok(service)
    }

    pub fn login(&mut self, nick: &str) -> io::Result<()> {
        try!(self.tcp_writer.write(format!("USER {} 0 * :IRC Envoy Client \r\n", nick).as_bytes()));
        try!(self.tcp_writer.write(format!("NICK {} \r\n", nick).as_bytes()));
        try!(self.tcp_writer.flush());
        Ok(())
    }

    pub fn join(&mut self, channel: &str) -> io::Result<()> {
        try!(self.tcp_writer.write(format!("JOIN #{} \r\n", channel).as_bytes()));
        try!(self.tcp_writer.write(format!("PRIVMSG #{} :Hi! \r\n", channel).as_bytes()));
        try!(self.tcp_writer.flush());
        Ok(())
    }
}
