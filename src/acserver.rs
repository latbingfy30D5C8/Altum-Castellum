// running the tor daemon is your responsibility
use crate::*;
use std::{
    io::{ErrorKind, Read},
    net::TcpListener,
};

const BINDING_ADDR: &str = "127.0.0.1:9050"; // change it if you want

impl ACServer {
    pub fn new() -> Result<ACServer> {
        let listener = TcpListener::bind(BINDING_ADDR)?;
        listener.set_nonblocking(true)?;
        Ok(ACServer {
            listener: listener,
            connections: Vec::new(),
        })
    }
    pub fn run(&mut self) -> Result<()> {
        // accept connections -> get message -> process message -> send messages -> repeat
        let mut i = 0;
        let mut logit_handle = None;
        loop {
            match self.listener.accept() {
                Err(b) if b.kind() == ErrorKind::WouldBlock => {}
                Ok(t) => {
                    t.0.set_nonblocking(true)?;
                    self.connections.push((t.0, String::new()));
                }
                Err(e) => {
                    let _ = logit2(e, &mut logit_handle);
                }
            }

            let (stream, message) = &mut self.connections.get_mut(i).ok_or("Failed with empty index but index considered and MUST always be valid on the Mr. Victor. Bad program flow. Fix it fucking immediately, please. Failed while reading message btw. Oh, and did i say you are a looser? Hahaha.")?;
            stream.read_to_string(message);

            
        }
    }
}
