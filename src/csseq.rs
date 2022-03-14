use std::io::Write;
use std::net::*;

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
