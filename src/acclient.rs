use crate::*;

use std::{
    env, fs,
    io::{self, ErrorKind, prelude::*},
    net::TcpStream,
    path::PathBuf,
    process::Command,
    time,
};

impl ACClient {
    // for anyone going to debug this bulky shit, patience be with you.
    pub fn new(
        irc_server_onion_addr: &str,
        nickname: &str,
        username: &str,
        realname: &str,
    ) -> Result<ACClient> {
        let tor_subprcss = Command::new("tor/tor")
            .args(["--ControlPort", "9051", "--CookieAuthentication", "1"])
            .spawn()?;
        let cookie_hex = read_cookie_file()?;
        let mut tor_control_stream = authenticate_tc_with_cookie(cookie_hex)?;
        let _ = wait_for_tor_bootstrap(&mut tor_control_stream);
        let mut ac_server_stream =
            socks::Socks5Stream::connect("127.0.0.1:9050", (irc_server_onion_addr, 6667))?;
        connection_registration(nickname, &mut ac_server_stream, username, realname)?;
        Ok(ACClient {
            tor: tor_subprcss,
            tor_control: tor_control_stream,
            onion_addr: irc_server_onion_addr.into(),
            server: ac_server_stream,
            nickname: nickname.into(),
            username: username.into(),
            realname: realname.into(),
        })
    }
}

fn connection_registration(
    nickname: &str,
    ac_server_stream: &mut Socks5Stream,
    username: &str,
    realname: &str,
) -> Result<()> {
    let nick_message = String::from("NICK") + " " + nickname + "\r\n";
    ac_server_stream.write_all(nick_message.as_bytes())?;
    let username_message = String::from("USER") + " " + username + " 0 * :" + realname + "\r\n";
    ac_server_stream.write_all(username_message.as_bytes())?;
    let mut replies = String::new();
    ac_server_stream.read_to_string(&mut replies)?;
    if !replies.contains("001") {
        return Err(io::Error::new(
            ErrorKind::ConnectionRefused,
            "i don't fucking know anything but server didn't replied with RPL_WELCOME.",
        )
        .into());
    }
    // handle the fucking error replies
    Ok(())
}

fn read_cookie_file() -> Result<String> {
    #[cfg(target_os = "windows")]
    let mut cookie_path = PathBuf::from(&env::var("appdata")?);
    #[cfg(target_os = "windows")]
    cookie_path.push("tor");
    #[cfg(target_os = "windows")]
    cookie_path.push("control_auth_cookie");
    #[cfg(target_os = "linux")]
    let cookie_path = "i dunno what to put here, yet";
    let mut cookie_hex = String::new();
    for byte in fs::read(cookie_path)?.into_iter() {
        // the tor control protocol wanting the authentication string(cookie thing) to be encoded in hexadecimal, idk why // update: I just realized i can just send the cookie quoted instead of turning it inot hex... fuck // update: it's for the escaping characters
        cookie_hex.push_str(&u8_to_hex(byte));
    }
    Ok(cookie_hex)
}

fn authenticate_tc_with_cookie(cookie_hex: String) -> Result<TcpStream> {
    let mut auth_message = String::from("AUTHENTICATE ");
    auth_message.push_str(&cookie_hex);
    auth_message.push_str("\r\n");
    let mut tor_control_stream = TcpStream::connect("127.0.0.1:9051")?;
    tor_control_stream.write_all(auth_message.as_bytes())?;
    tor_control_stream.flush()?;
    let mut tc_auth_reply = String::new();
    let _ = tor_control_stream.read_to_string(&mut tc_auth_reply);
    if !tc_auth_reply.starts_with("250") {
        // "250 OK" // "515 Bad authentication" (tcp connection closes when 515 btw)
        return Err(Box::new(io::Error::from(ErrorKind::PermissionDenied)));
    }
    Ok(tor_control_stream)
}

#[allow(unused)]
fn wait_for_tor_bootstrap(tor_control_stream: &mut TcpStream) -> Result<()> {
    let time_limit = time::Instant::now();
    loop {
        tor_control_stream.write_all(b"GETINFO status/circuit-established\r\n");
        tor_control_stream.flush();
        let mut tc_circuit_getinfo_reply = String::new();
        tor_control_stream.read_to_string(&mut tc_circuit_getinfo_reply);
        if tc_circuit_getinfo_reply.contains("status/circuit-established=1") {
            return Ok(());
        }
        if time_limit.elapsed().as_secs() > 120 {
            return Err(io::Error::from(ErrorKind::TimedOut).into());
        }
    }
}
