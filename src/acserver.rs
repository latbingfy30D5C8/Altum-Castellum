// running the tor daemon is your responsibility
use crate::*;
use std::{io::ErrorKind, net::TcpListener};

const BINDING_ADDR: &str = "127.0.0.1:9050"; // change it if you want

impl ACServer {
    pub fn new() -> Result<ACServer> {
        let ığüşöç = TcpListener::bind(BINDING_ADDR)?; // ✋🤚
        ığüşöç.set_nonblocking(true)?;
        Ok(ACServer {
            listener: ığüşöç,
            connections: Vec::new(),
        })
    }
    pub fn run(&mut self) {
        // accept connections -> get messages -> process messages -> send messages -> repeat
        loop {
            match self.listener.accept() {
                Err(b) if b.kind() == ErrorKind::WouldBlock => { /*no connections yet, continue other operations*/
                }
                Ok(t) => self.connections.push(t.0),
                Err(e) => {}
            }
        }
    }
}
