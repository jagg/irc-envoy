use std::str;
use nom::IResult;

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
    Other(String),
    Ping(Server, String),
}

pub fn parse(input: &[u8]) -> Msg {
    let result = irc_parser(input);
    match result {
        IResult::Done(_, parsed) => parsed,
        _ => Msg::Other(str::from_utf8(input).unwrap().to_string()),
    }
}

named!(irc_parser< &[u8], Msg >, alt!(chan_content | priv_content));

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

named!(destination_user<&[u8], &str>,
    map_res!(
        chain!(
           char!(' ')               ~
           user: take_until!(" ")   ~
           char!(' ')               ,
           || {user} ),
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
        char!(':')                       ~
        msg: message,
     ||{Msg::ChanContent { origin: User(nick.to_string()), destination: Channel(channel.to_string()), msg: msg.to_string()} }
    )
);


named!(priv_content<&[u8], Msg>,
    chain!(
        nick: user_name                 ~
        take_until_and_consume!(" ")    ~
        tag!("PRIVMSG")                 ~
        destination: destination_user   ~
        char!(':')                      ~
        msg: message,
        ||{Msg::PrivContent { origin: User(nick.to_string()), destination: User(destination.to_string()), msg: msg.to_string()} }
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

    #[test]
    fn test_priv_message() {
        let msg = Msg::PrivContent {
            origin: User("Angel".to_string()),
            destination: User("Wiz".to_string()),
            msg: "Are you receiving this message ?".to_string(),
        };
        let parsed_cnt =
            parse(b":Angel!wings@irc.org PRIVMSG Wiz :Are you receiving this message ?\r\n");
        assert_eq!(msg, parsed_cnt);
    }


}
