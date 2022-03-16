use std::collections::VecDeque;
use std::io::{Read, Write, stdout};
use std::net;
use std::thread;

fn get_count() -> Vec<u8> {
    let mut stream = net::TcpStream::connect("127.0.0.1:10123").unwrap();
    let mut buf = vec![0u8; 22];
    stream.read_to_end(&mut buf).unwrap();
    buf
}

fn print_count(h: thread::JoinHandle<Vec<u8>>) {
    let mut buf = h.join().unwrap();
    let nbuf = buf.len();
    buf[nbuf - 2] = b'\n';
    stdout().write_all(&buf[..nbuf-1]).unwrap();
}

pub fn send(n: usize, m: usize) {
    let mut handles = VecDeque::with_capacity(2 * m);
    for _ in 0..n {
        handles.push_front(thread::spawn(get_count));
        if handles.len() > m {
            for h in handles.drain(m..) {
                print_count(h);
            }
        }
    }
    for h in handles.into_iter() {
        print_count(h);
    }
}
