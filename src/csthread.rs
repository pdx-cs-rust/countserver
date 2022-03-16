use std::collections::VecDeque;
use std::io::Write;
use std::net::*;

pub mod simple {

    use super::*;

    use std::sync::atomic::{AtomicU64, Ordering};

    pub fn start(m: usize) {
        let counter = AtomicU64::new(0);
        let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
        std::thread::scope(|spawner| {
            let mut handles = VecDeque::with_capacity(2 * m);
            loop {
                let (mut socket, _addr) = listener.accept().unwrap();
                let counter = &counter;
                let h = spawner.spawn(move || {
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
        });
    }
}

pub mod fast {

    use super::*;

    use std::sync::Mutex;

    use crate::counter::Counter;

    pub fn start(m: usize) {
        let counter = Mutex::new(Counter::default());
        let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
        std::thread::scope(|spawner| {
            let mut handles = VecDeque::with_capacity(2 * m);
            loop {
                let (mut socket, _addr) = listener.accept().unwrap();
                let counter = &counter;
                let h = spawner.spawn(move || {
                    let mut counter = counter.lock().unwrap();
                    socket.write_all(counter.value()).unwrap();
                    counter.inc();
                    drop(counter);
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
        });
    }
}
