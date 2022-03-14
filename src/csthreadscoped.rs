use std::io::Write;
use std::net::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn start(m: usize) {
    let counter = AtomicU64::new(0);
    let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
    std::thread::scope(|s| {
        for _ in 0..m {
            s.spawn(|| {
                for socket in listener.incoming() {
                    let mut socket = socket.unwrap();
                    let count = counter.fetch_add(1, Ordering::SeqCst);
                    write!(socket, "{}\r\n", count).unwrap();
                    socket.flush().unwrap();
                    drop(socket);
                }
            });
        }
    });
}
