use std::fmt;
use std::convert::From;


#[derive(Eq,Debug,PartialEq)]
pub struct Channel(String);

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for Channel {
    fn from(val: T) -> Self {
        Channel(val.into())
    }
}

#[derive(Eq,Debug,PartialEq)]
pub struct User(String);

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for User {
    fn from(val: T) -> Self {
        User(val.into())
    }
}

#[derive(Eq,Debug,PartialEq)]
pub struct Server(String);

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Eq,Debug,PartialEq)]
pub enum Msg {
    Join(Channel),
    Part(Channel),
    PrivContent {
        origin: User,
        destination: User,
        msg: String,
    },
    ChanContent {
        origin: User,
        destination: Channel,
        msg: String,
    },
    Other(String),
    Ping(Server, String),
}
