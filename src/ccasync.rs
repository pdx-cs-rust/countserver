pub mod rt_async_std {
    pub use async_std::io::{self, ReadExt, WriteExt};
    pub use async_std::net::TcpStream;
    pub use async_std::task;

    async fn drop_handle(h: task::JoinHandle<()>) {
        h.await;
    }

    super::ccasync!();

    pub fn start(n: usize, m: usize) {
        task::block_on(send(n, m));
    }
}

pub mod rt_tokio {
    pub use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
    pub use tokio::net::TcpStream;
    pub use tokio::runtime::Runtime;
    pub use tokio::task;

    async fn drop_handle(h: task::JoinHandle<()>) {
        h.await.unwrap();
    }

    super::ccasync!();

    pub fn start(n: usize, m: usize) {
        let rt = Runtime::new().unwrap();
        rt.block_on(send(n, m));
    }
}

macro_rules! ccasync {
    () => {
        async fn get_count() -> u64 {
            let mut stream = TcpStream::connect("127.0.0.1:10123").await.unwrap();
            let mut buf = String::with_capacity(22);
            stream.read_to_string(&mut buf).await.unwrap();
            drop(stream);
            buf.trim_end().parse().unwrap()
        }

        async fn send(n: usize, m: usize) {
            let mut handles = Vec::with_capacity(m);
            for _ in 0..n {
                let h = task::spawn(async {
                    let c = get_count().await;
                    io::stdout()
                        .write_all(format!("{}\n", c).as_bytes())
                        .await
                        .unwrap();
                });
                handles.push(h);
                if handles.len() >= m {
                    for h in handles.drain(..) {
                        drop_handle(h).await;
                    }
                }
            }
            for h in handles {
                drop_handle(h).await;
            }
        }
    };
}
use ccasync;
