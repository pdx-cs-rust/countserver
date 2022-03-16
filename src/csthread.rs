use std::io::Write;
use std::collections::VecDeque;
use std::net::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn start(m: usize) {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
    let mut handles = VecDeque::with_capacity(2 * m);
    loop {
        let (mut socket, _addr) = listener.accept().unwrap();
        let counter = &COUNTER;
        let h = std::thread::spawn(move || {
            let count = counter.fetch_add(1, Ordering::SeqCst);
            write!(socket, "{}\r\n", count).unwrap();
            socket.flush().unwrap();
            drop(socket);
        });
        handles.push_front(h);
        if handles.len() > m {
            for h in handles.drain(m..) {
                h.join().unwrap();
            }
        }
    }
}
