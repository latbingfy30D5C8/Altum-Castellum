// i like talking with myself
// for tests, put tor\tor.exe into root
use socks::Socks5Stream;
use std::{
    net::{TcpListener, TcpStream},
    process::Child,
};
pub use utils::*;

mod acclient;
mod acserver;
#[cfg(test)]
mod tests;
mod utils;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

pub struct ACClient {
    tor: Child,
    tor_control: TcpStream,
    onion_addr: String,
    server: Socks5Stream,
    nickname: String,
    username: String,
    realname: String,
}

pub struct ACServer {
    listener: TcpListener,
    connections: Vec<TcpStream>,
}
