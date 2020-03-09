use async_std::net::*;
use async_std::prelude::*;
use async_std::task;
use async_std::sync::Mutex;
use std::sync::Arc;

async fn reply(mut socket: TcpStream, counter: Arc<Mutex<u64>>) {
    let mut counter = counter.lock().await;
    let count = *counter;
    *counter += 1;
    socket.write_all(format!("{}\r\n", count).as_bytes()).await.unwrap();
}


async fn listen(counter: Arc<Mutex<u64>>) {
    let listener = TcpListener::bind("127.0.0.1:10123").await.unwrap();
    let mut incoming = listener.incoming();
    while let Some(socket) = incoming.next().await {
        let socket = socket.unwrap();
        let addr = socket.peer_addr().unwrap();
        eprintln!("new client: {:?}", addr);
        task::spawn(reply(socket, counter.clone()));
    }
}

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    task::block_on(listen(counter.clone()));
}
