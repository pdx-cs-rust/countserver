use std::io::Write;
use std::net::*;

pub mod simple {

    use super::*;

    pub fn start() {
        let mut counter = 0u64;
        let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
        loop {
            let (mut socket, _addr) = listener.accept().unwrap();
            counter += 1;
            write!(socket, "{}\r\n", counter).unwrap();
            socket.flush().unwrap();
            drop(socket);
        }
    }
}

pub mod fast {

    use super::*;

    use crate::counter::Counter;

    pub fn start() {
        let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
        let mut counter = Counter::default();
        loop {
            let (mut socket, _addr) = listener.accept().unwrap();
            let _ = socket.write_all(counter.value()).unwrap();
            counter.inc();
            socket.flush().unwrap();
            drop(socket);
        }
    }
}
