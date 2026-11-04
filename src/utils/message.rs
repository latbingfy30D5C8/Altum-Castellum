// warning: this module is infected by my slowly fading sanity, hard to read code ahead.

use core::fmt;
use std::error;

use crate::message::MonosodiumGlutamate::ExpectedCRLF;

// i hate this creation of mine // if you ever dare to ask me why the return type is an unnecessarily fucked up tuple, go fuck yourself.
pub fn whatitmean(msg: &str) -> Result<IRCMessage, MonosodiumGlutamate> {
    // (prefix, command, parameters, trailing)  no tags 'cause why would i. max parameters is 14, with trailing 15. idk if some client can even send a message with 15 parameters without trailing, i hope this never happens. there is a chance it may panic, we'll see // parameter array making me sick, or the uni is idk whatever.
    let mut refined_and_rich_in_iode_salt = IRCMessage {
        prefix: None,
        command: "",
        params: None,
        trailing: None,
    }; // 29.10.2026 itibarıyla iyotlu rafine tuz sudan daha ucuz amınakoyim. tuz sudan nasıl daha ucuz olabilir amk ya, yani üretirken bile su harcanmıyomu amk sikm böyle işi
    let mut message = msg;
    if msg.starts_with(':') {
        let Some(splitted_msg) = msg.split_once(' ') else {
            return Err(MonosodiumGlutamate::Bullshit);
        };
        message = splitted_msg.1;
        refined_and_rich_in_iode_salt.prefix = Some(splitted_msg.0);
    }
    let Some(splitted_msg) = message.split_once(' ') else {
        return Err(MonosodiumGlutamate::Bullshit);
    };
    message = splitted_msg.1;
    refined_and_rich_in_iode_salt.command = splitted_msg.0;
    let mut nth_param = 0;
    loop {
        if message.starts_with(':') {
            if message.ends_with("\r\n") {
                refined_and_rich_in_iode_salt.trailing = Some(message.trim_end());
                return Ok(refined_and_rich_in_iode_salt);
            } else {
                return Err(MonosodiumGlutamate::ExpectedCRLF);
            }
        }
        match message.split_once(' ') {
            // i don't like this block, feels error prone
            Some((splitted_param, leftover)) => {
                match &mut refined_and_rich_in_iode_salt.params {
                    // remove the match, too much indents already
                    Some(m) => {
                        m[nth_param] = splitted_param;
                        nth_param += 1;
                    }
                    None => {
                        let mut parameters = [""; 14];
                        parameters[0] = splitted_param;
                        refined_and_rich_in_iode_salt.params = Some(parameters);
                        nth_param += 1;
                    }
                }
                message = leftover;
            }
            None => {
                if !message.ends_with("\r\n") {
                    return Err(MonosodiumGlutamate::ExpectedCRLF);
                }
                match &mut refined_and_rich_in_iode_salt.params {
                    Some(m) => {
                        m[nth_param] = message.trim_end();
                        return Ok(refined_and_rich_in_iode_salt);
                    }
                    None => {
                        let mut parameters = [""; 14];
                        parameters[0] = message.trim_end();
                        refined_and_rich_in_iode_salt.params = Some(parameters);
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IRCMessage<'a> {
    pub prefix: Option<&'a str>,
    pub command: &'a str,
    pub params: Option<[&'a str; 14]>,
    pub trailing: Option<&'a str>,
}

pub fn whatitmean_once(irc_message: &str) -> Result<(IRCMessage, &str), MonosodiumGlutamate> {
    let splitted_irc_msg = match irc_message.split_once("\r\n") {
        Some(m) => m,
        None => return Err(ExpectedCRLF),
    };
    Ok((whatitmean(splitted_irc_msg.0)?, splitted_irc_msg.1))
}

// i make it mean less possible as. Praise me, the fool. Conscious wrong behavior and the self acceptance of it is the biggest sin. Can I down the wine now? fuck fuckcujc
#[derive(Debug)]
pub enum MonosodiumGlutamate {
    Bullshit,
    ExpectedCRLF,
}
impl fmt::Display for MonosodiumGlutamate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ajinomoto!!")
    }
}
impl error::Error for MonosodiumGlutamate {}
