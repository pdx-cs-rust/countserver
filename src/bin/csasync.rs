#[cfg(feature = "async-std")]
mod async_rt {
    pub use async_std::net::*;
    pub use async_std::prelude::*;
    pub use async_std::task;
    pub use async_std::sync::Arc;
    pub use std::sync::atomic::{AtomicU64, Ordering};
}

use async_rt::*;

async fn reply(mut socket: TcpStream, counter: Arc<AtomicU64>) {
    let count = counter.fetch_add(1, Ordering::SeqCst);
    socket.write_all(format!("{}\r\n", count).as_bytes()).await.unwrap();
}


async fn listen() {
    let counter = Arc::new(AtomicU64::new(0));
    let listener = TcpListener::bind("127.0.0.1:10123").await.unwrap();
    let mut incoming = listener.incoming();
    while let Some(socket) = incoming.next().await {
        let socket = socket.unwrap();
        //let addr = socket.peer_addr().unwrap();
        //eprintln!("new client: {:?}", addr);
        let counter = Arc::clone(&counter);
        task::spawn(reply(socket, counter));
    }
}

fn main() {
    task::block_on(listen());
}
