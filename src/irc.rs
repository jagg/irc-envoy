use std::time::Duration;
use std::net;
use std::io;
use std::io::{BufReader, BufRead, BufWriter};
use std::io::prelude::{Write, Read};
use std::thread;
use std::time;

pub struct Config {
    pub server: String,
    pub port: u16,
}

pub struct IrcReceiver {
    tcp_stream: net::TcpStream,
}

impl IrcReceiver {
    pub fn receive_and_print(&mut self) -> thread::JoinHandle<()> {
        let stream = self.tcp_stream.try_clone().unwrap();
        let mut stream = BufReader::new(stream);
        thread::spawn(move || {
            loop {
                let mut buffer = String::new();
                let num_bytes = stream.read_line(&mut buffer).unwrap();
                println!("[{:?}] CONNECTED: {}", num_bytes, buffer);
            }
        })

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
    receiver: IrcReceiver,
    sender: IrcSender,
}


fn new(con: Config) -> io::Result<(IrcReceiver, IrcSender)> {
    let stream = try!(net::TcpStream::connect((con.server.as_str(), con.port)));
    let reader = stream.try_clone().unwrap();
    let buf_writer = BufWriter::new(stream.try_clone().unwrap());
    Ok((IrcReceiver { tcp_stream: reader }, IrcSender { tcp_stream: buf_writer }))
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
