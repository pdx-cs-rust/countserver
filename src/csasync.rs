use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);
const ADDR: &str = "127.0.0.1:10123";

pub mod rt_async_std {
    pub use async_std::net::*;
    pub use async_std::prelude::*;
    pub use async_std::task;

    super::reply!();

    async fn listen() {
        let listener = TcpListener::bind(super::ADDR).await.unwrap();
        let mut incoming = listener.incoming();
        super::serve!(incoming);
    }

    pub fn start() {
        task::block_on(listen());
    }
}

pub mod rt_tokio {
    pub use tokio::io::{self, AsyncWriteExt};
    pub use tokio::net::{TcpListener, TcpStream};
    pub use tokio::runtime::Runtime;
    pub use tokio::task;

    use async_stream::try_stream;
    pub use futures_core::stream::Stream;
    pub use futures_util::pin_mut;
    pub use futures_util::stream::StreamExt;

    pub fn incoming_stream(listener: TcpListener) -> impl Stream<Item = io::Result<TcpStream>> {
        try_stream! {
            loop {
                let (ts, _) = listener.accept().await?;
                yield ts;
            }
        }
    }

    super::reply!();

    async fn listen() {
        let listener = TcpListener::bind(super::ADDR).await.unwrap();
        let incoming = incoming_stream(listener);
        pin_mut!(incoming);
        super::serve!(incoming);
    }

    pub fn start() {
        let rt = Runtime::new().unwrap();
        rt.block_on(listen());
    }
}

macro_rules! reply {
    () => {
        async fn reply(mut socket: TcpStream, counter: &super::AtomicU64) {
            let count = counter.fetch_add(1, super::Ordering::SeqCst);
            socket
                .write_all(format!("{}\r\n", count).as_bytes())
                .await
                .unwrap();
            socket.flush().await.unwrap();
            drop(socket);
        }
    };
}
use reply;

macro_rules! serve {
    ($incoming:ident) => {
        while let Some(socket) = $incoming.next().await {
            let socket = socket.unwrap();
            task::spawn(reply(socket, &super::COUNTER));
        }
    };
}
use serve;
