use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "async-std-rt")]
mod async_rt {
    pub use async_std::prelude::*;
    pub use async_std::net::*;
    pub use async_std::task;
}

#[cfg(feature = "tokio-rt")]
mod async_rt {
    pub use tokio::io::{self, AsyncWriteExt};
    pub use tokio::net::{TcpStream, TcpListener};
    pub use tokio::task;
    pub use tokio::runtime::Runtime;
    pub use std::sync::Arc;

    use async_stream::try_stream;
    pub use futures_core::stream::Stream;
    pub use futures_util::stream::StreamExt;
    pub use futures_util::pin_mut;

    pub fn incoming_stream(listener: TcpListener) -> impl Stream<Item = io::Result<TcpStream>> {
        try_stream! {
            loop {
                let (ts, _) = listener.accept().await?;
                yield ts;
            }
        }
    }
}

use async_rt::*;

async fn reply(mut socket: TcpStream, counter: &AtomicU64) {
    let count = counter.fetch_add(1, Ordering::SeqCst);
    socket.write_all(format!("{}\r\n", count).as_bytes()).await.unwrap();
    socket.flush().await.unwrap();
    drop(socket);
}


async fn listen() {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let addr = "127.0.0.1:10123";
    let listener = TcpListener::bind(addr).await.unwrap();
    #[cfg(feature = "async-std-rt")]
    let mut incoming = listener.incoming();
    #[cfg(feature = "tokio-rt")]
    let incoming = incoming_stream(listener);
    #[cfg(feature = "tokio-rt")]
    pin_mut!(incoming);
    while let Some(socket) = incoming.next().await {
        let socket = socket.unwrap();
        task::spawn(reply(socket, &COUNTER));
    }
}

fn main() {
    #[cfg(feature = "tokio-rt")] {
        let rt = Runtime::new().unwrap();
        rt.block_on(listen());
    }
    #[cfg(feature = "async-std-rt")]
    task::block_on(listen());
}
