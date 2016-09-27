use std::str;

#[derive(Eq,Debug,PartialEq)]
pub struct Channel(String);

#[derive(Eq,Debug,PartialEq)]
pub struct User(String);

#[derive(Eq,Debug,PartialEq)]
pub struct Server(String);


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

    Ping(Server, String),
}

pub fn parse(input: &[u8]) -> Msg {
    let (rest, r) = chan_content(input).unwrap();
    println!("{}", str::from_utf8(rest).unwrap());
    r
}
// Examples
// [74] CONNECTED: :jagg!uid183337@moz-ht3idd.brockwell.irccloud.com PRIVMSG #irc-test :hi!
// [99] CONNECTED: :jagg!uid183337@moz-ht3idd.brockwell.irccloud.com PRIVMSG #irc-test :irc-envoy-test: did it work?
//                 :Angel!wings@irc.org PRIVMSG Wiz :Are you receiving this message ?

// :{nick-sender}!{host} PRIVMSG {opt: nick-receiver / #channel} :{message}

named!(user_name<&[u8], &str>,
    map_res!(
        chain!(
            nick: delimited!(char!(':'), is_not!("!"), char!('!')),
            || {nick} ),
        str::from_utf8
    )
);

named!(channel_name<&[u8], &str>,
    map_res!(
        chain!(
            channel: delimited!(char!('#'), is_not!(" "), char!(' ')),
            || {channel} ),
        str::from_utf8
    )
);

named!(message<&[u8], &str>,
    map_res!(
        chain!(
            msg: take_until!("\r"),
            || {msg} ),
        str::from_utf8
    )
);

named!(chan_content<&[u8], Msg>,
    chain!(
        nick: user_name                 ~
        take_until_and_consume!(" ")    ~
        tag!("PRIVMSG ")                ~
        channel: channel_name           ~
        tag!(":")                        ~
        msg: message,
     ||{Msg::ChanContent { origin: User(nick.to_string()), destination: Channel(channel.to_string()), msg: msg.to_string()} }
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_message() {
        let msg = Msg::ChanContent {
            origin: User("jagg".to_string()),
            destination: Channel("irc-test".to_string()),
            msg: "hi!".to_string(),
        };
        assert_eq!(msg, parse(b":jagg!uid183337@moz-ht3idd.brockwell.irccloud.com PRIVMSG #irc-test :hi!\r\n"));
    }
}
