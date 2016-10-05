use std::time::Duration;
use std::net;
use std::io;
use std::io::{BufReader, BufRead, BufWriter};
use std::io::prelude::{Write, Read};
use std::thread;
use std::time;

use super::input;

pub struct Config {
    pub server: String,
    pub port: u16,
}

pub struct IrcBackground {
    tcp_stream: net::TcpStream,
    tcp_writer: BufWriter<net::TcpStream>,
}


impl IrcBackground {
    pub fn receive_and_print(&mut self) -> thread::JoinHandle<()> {
        let stream = self.tcp_stream.try_clone().unwrap();
        let mut stream = BufReader::new(stream);
        thread::spawn(move || {
            loop {
                let mut buffer = String::new();
                let num_bytes = stream.read_line(&mut buffer).unwrap();
                let msg = input::parse(buffer.as_bytes());
                simple_print(msg);
            }
        })

    }
}

fn simple_print(msg: input::Msg) {
    match msg {
        input::Msg::PrivContent { origin: o, destination: d, msg } => {
            println!("[{} -> {}] {}", o, d, msg)
        }
        input::Msg::ChanContent { origin: o, destination: d, msg } => {
            println!("[{} -> #{}] {}", o, d, msg)
        }
        input::Msg::Other(msg) => println!("... {}", msg),
        _ => println!("This should't happen!"),
    }
}

pub struct IrcSender {
    tcp_stream: BufWriter<net::TcpStream>,
}

impl IrcSender {
    pub fn login(&mut self, nick: &str) {
        self.tcp_stream.write(format!("USER {} 0 * :IRC Envoy Client \r\n", nick).as_bytes());
        self.tcp_stream.write(format!("NICK {} \r\n", nick).as_bytes());
        self.tcp_stream.flush();
    }

    pub fn join(&mut self, channel: &str) {
        self.tcp_stream.write(format!("JOIN #{} \r\n", channel).as_bytes());
        self.tcp_stream.write(format!("PRIVMSG #{} :Hi! \r\n", channel).as_bytes());
        self.tcp_stream.flush();
    }
}

pub struct Irc {
    receiver: IrcBackground,
    sender: IrcSender,
}


fn new(con: Config) -> io::Result<(IrcBackground, IrcSender)> {
    let stream = try!(net::TcpStream::connect((con.server.as_str(), con.port)));
    let reader = stream.try_clone().unwrap();
    let buf_writer = BufWriter::new(stream.try_clone().unwrap());
    let back_writer = BufWriter::new(stream.try_clone().unwrap());
    Ok((IrcBackground {
        tcp_stream: reader,
        tcp_writer: back_writer,
    },
        IrcSender { tcp_stream: buf_writer }))
}

impl Irc {
    pub fn connect(con: Config) -> io::Result<Self> {
        let (mut reader, mut sender) = try!(new(con));
        sender.login("irc-envoy-test");
        reader.receive_and_print();
        thread::sleep(time::Duration::from_millis(2000));
        sender.join("irc-test");

        Ok(Irc {
            receiver: reader,
            sender: sender,
        })
    }
}
