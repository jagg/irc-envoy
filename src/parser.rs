use std::str;
use nom::IResult;

use super::data;

pub fn parse(input: &[u8]) -> data::Msg {
    let result = irc_parser(input);
    match result {
        IResult::Done(_, parsed) => parsed,
        _ => data::Msg::Other(str::from_utf8(input).unwrap().to_string()),
    }
}

named!(irc_parser< &[u8], data::Msg >, alt!(chan_content | priv_content));

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

named!(chan_content<&[u8], data::Msg>,
    chain!(
        nick: user_name                 ~
        take_until_and_consume!(" ")    ~
        tag!("PRIVMSG ")                ~
        channel: channel_name           ~
        char!(':')                      ~
        msg: message,
     ||{data::Msg::ChanContent { origin: data::User::from(nick), destination: data::Channel::from(channel), msg: msg.to_string()} }
    )
);


named!(priv_content<&[u8], data::Msg>,
    chain!(
        nick: user_name                 ~
        take_until_and_consume!(" ")    ~
        tag!("PRIVMSG")                 ~
        destination: destination_user   ~
        char!(':')                      ~
        msg: message,
        ||{data::Msg::PrivContent { origin: data::User::from(nick), destination: data::User::from(destination), msg: msg.to_string()} }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::data;

    #[test]
    fn test_channel_message() {
        let msg = data::Msg::ChanContent {
            origin: data::User::from("jagg"),
            destination: data::Channel::from("irc-test"),
            msg: "hi!".to_string(),
        };
        assert_eq!(msg, parse(b":jagg!uid183337@moz-ht3idd.brockwell.irccloud.com PRIVMSG #irc-test :hi!\r\n"));
    }

    #[test]
    fn test_priv_message() {
        let msg = data::Msg::PrivContent {
            origin: data::User::from("Angel"),
            destination: data::User::from("Wiz"),
            msg: "Are you receiving this message ?".to_string(),
        };
        let parsed_cnt =
            parse(b":Angel!wings@irc.org PRIVMSG Wiz :Are you receiving this message ?\r\n");
        assert_eq!(msg, parsed_cnt);
    }


}
