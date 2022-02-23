use std::io::Read;
use std::net;
use std::thread;

fn get_count() -> u64 {
    let mut stream = net::TcpStream::connect("127.0.0.1:10123").unwrap();
    let mut buf = String::with_capacity(22);
    stream.read_to_string(&mut buf).unwrap();
    buf.trim_end().parse().unwrap()
}


fn send(n: usize) {
    let mut handles = Vec::with_capacity(n);
    for _ in 0..n {
        handles.push(thread::spawn(get_count));
    }
    for h in handles {
        println!("{:?}", h.join().unwrap());
    }
}

fn main() {
    let n = std::env::args().nth(1).unwrap().parse().unwrap();
    send(n);
}
