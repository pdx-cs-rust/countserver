use std::io::Read;
use std::net;

pub fn send(n: usize) {
    let mut buf = String::with_capacity(22);
    for _ in 0..n {
        let mut stream = net::TcpStream::connect("127.0.0.1:10123").unwrap();
        buf.clear();
        stream.read_to_string(&mut buf).unwrap();
        let count: u64 = buf.trim_end().parse().unwrap();
        println!("{}", count);
    }
}
