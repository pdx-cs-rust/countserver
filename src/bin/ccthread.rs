use std::io::Read;
use std::net;
use std::thread;

fn get_count() -> u64 {
    let mut stream = net::TcpStream::connect("127.0.0.1:10123").unwrap();
    let mut buf = String::with_capacity(22);
    stream.read_to_string(&mut buf).unwrap();
    buf.trim_end().parse().unwrap()
}


fn send(n: usize, m: usize) {
    let mut handles = Vec::with_capacity(m);
    for _ in 0..n {
        handles.push(thread::spawn(get_count));
        if handles.len() >= m {
            for h in handles.drain(..) {
                println!("{:?}", h.join().unwrap());
            }
        }
    }
    for h in handles {
        println!("{:?}", h.join().unwrap());
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let n = args
        .next()
        .expect("Usage: ccthread count [concurrency_limit]")
        .parse()
        .expect("Couldn't parse count as integer");
    let m = args
        .next()
        .as_deref()
        .unwrap_or("100")
        .parse()
        .expect("Couldn't parse concurrency limit as integer");
    send(n, m);
}
