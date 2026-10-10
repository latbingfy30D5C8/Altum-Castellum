// i like talking with myself
// for tests, put tor\tor.exe into root
use std::{
    env, fs,
    io::prelude::*,
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command},
};
#[cfg(test)]
mod tests;
mod tor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct ACClient {
    tor: Child,
}

impl ACClient {
    pub fn new() -> Result<ACClient> {
        let mut tor_subprcss = Command::new("tor/tor")
            .args(["--ControlPort", "9051", "--CookieAuthentication", "1"])
            .spawn()?;
        #[cfg(target_os = "windows")]
        let mut cookie_path = PathBuf::from(&env::var("appdata")?); // specifically AppData\Roaming
        #[cfg(target_os = "windows")]
        cookie_path.push("tor"); // dir
        #[cfg(target_os = "windows")]
        cookie_path.push("control_auth_cookie"); // file
        #[cfg(target_os = "linux")]
        let cookie_path = "i dunno what to put here, yet";
        let mut cookie_hex = String::new();
        for num in fs::read(cookie_path)?.into_iter() {
            // the tor control protocol wanting the authentication string(cookie thing) to encoded in hexadecimal, idk why // update: I just realized i can just send the cookie quoted instead of turning it inot hex... fuck // update: it's for the escaping characters
            cookie_hex.push_str(&u8_to_hex(num));
        }
        let mut tor_control_stream = TcpStream::connect("127.0.0.1:9051")?;
        let mut auth_message = String::from("AUTHENTICATE ");
        auth_message.push_str(&cookie_hex);
        auth_message.push_str(" \r\n");
        tor_control_stream.write_all(auth_message.as_bytes())?; // not sure if i should return the error from these
        tor_control_stream.flush()?;
        let mut tc_reply = String::new();
        tor_control_stream.read_to_string(&mut tc_reply);
        if tc_reply.starts_with("250") { // 250 OK
        } else if tc_reply.starts_with("515") { // 515 Bad authentication, connection closed btw
        }
        Ok(ACClient { tor: tor_subprcss })
    }
}

fn u8_to_hex(byte: u8) -> String {
    let mut hex = String::new();
    let first_4_digit = byte & 0xF;
    let last_4_digit = byte >> 4;
    if last_4_digit != 0 {
        // i believe there is a way to remove the if block but i didn't figure it out yet
        if last_4_digit < 10 {
            String::push(&mut hex, (last_4_digit + 48) as char);
        } else {
            String::push(&mut hex, (last_4_digit + 87) as char);
        }
    }
    if first_4_digit < 10 {
        String::push(&mut hex, (first_4_digit + 48) as char);
    } else {
        String::push(&mut hex, (first_4_digit + 87) as char);
    }
    hex
}
