use std::io::Write;
use std::net::*;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
    loop {
        let (mut socket, _addr) = listener.accept().unwrap();
        //eprintln!("new client: {:?}", addr);
        let counter = Arc::clone(&counter);
        let _ = std::thread::spawn(move || {
            let count = counter.fetch_add(1, Ordering::SeqCst);
            write!(socket, "{}\r\n", count).unwrap();
        });
    }
}
