// running the tor daemon is your responsibility
use crate::{message::whatitmean_once, *};
use std::{io::{ErrorKind, Read}, net::TcpListener};

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
        let mut message = String::new();
        let mut i = 0;
        loop {
            match self.listener.accept() {
                Err(b) if b.kind() == ErrorKind::WouldBlock => {}
                Ok(t) => {t.0.set_nonblocking(true)?; self.connections.push(t.0);}
                Err(e) => {return Err(e.into())}
            }
            self.connections[0].read_to_string(&mut message)?;
        }
    }
}