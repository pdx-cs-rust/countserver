#![feature(scoped_threads)]

use std::io::Write;
use std::net::*;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let counter = AtomicU64::new(0);
    let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
    let nthreads: usize = std::thread::available_parallelism().unwrap().into();
    std::thread::scope(|s| {
        for _ in 0..nthreads {
            s.spawn(|_| {
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
