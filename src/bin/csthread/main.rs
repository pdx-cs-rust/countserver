use std::io::Write;
use std::net::*;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
    loop {
        let (mut socket, addr) = listener.accept().unwrap();
        eprintln!("new client: {:?}", addr);
        let counter = counter.clone();
        let _ = std::thread::spawn(move || {
            let mut counter = counter.lock().unwrap();
            let count = *counter;
            *counter += 1;
            writeln!(socket, "{}", count).unwrap();
        });
    }
}
