use std::io::{Read, Write, stdout};
use std::net;

pub fn send(n: usize) {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut buf = vec![0u8; 22];
    for _ in 0..n {
        let mut stream = net::TcpStream::connect("127.0.0.1:10123").unwrap();
        buf.clear();
        stream.read_to_end(&mut buf).unwrap();
        let nbuf = buf.len();
        buf[nbuf - 2] = b'\n';
        stdout.write_all(&buf[..nbuf-1]).unwrap();
    }
}
