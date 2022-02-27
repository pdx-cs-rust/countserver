#![feature(thread_is_running)]

extern crate args;

use std::io::Write;
use std::net::*;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let args = args::get_args();
    let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
    let mut children = Vec::new();
    loop {
        let (mut socket, _addr) = listener.accept().unwrap();
        let counter = &COUNTER;
        let handle = std::thread::spawn(move || {
            let count = counter.fetch_add(1, Ordering::SeqCst);
            write!(socket, "{}\r\n", count).unwrap();
            socket.flush().unwrap();
            drop(socket);
        });
        children.push(handle);
        // XXX Clippy false-positive on the `filter_map()`. See
        // https://github.com/rust-lang/rust-clippy/issues/4433
        #[allow(clippy::unnecessary_filter_map)]
        while children.len() >= args.m / 2 {
            children = children
                .into_iter()
                .filter_map(|h| {
                    if h.is_running() {
                        Some(h)
                    } else {
                        let () = h.join().unwrap();
                        None
                    }
                })
                .collect();
        }
    }
}
