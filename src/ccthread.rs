use std::collections::VecDeque;
use std::io::Read;
use std::net;
use std::thread;

fn get_count() -> u64 {
    let mut stream = net::TcpStream::connect("127.0.0.1:10123").unwrap();
    let mut buf = String::with_capacity(22);
    stream.read_to_string(&mut buf).unwrap();
    buf.trim_end().parse().unwrap()
}

pub fn send(n: usize, m: usize) {
    let mut handles = VecDeque::with_capacity(2 * m);
    for _ in 0..n {
        handles.push_front(thread::spawn(get_count));
        if handles.len() > m {
            for h in handles.drain(m..) {
                println!("{:?}", h.join().unwrap());
            }
        }
    }
    for h in handles.into_iter() {
        println!("{:?}", h.join().unwrap());
    }
}
