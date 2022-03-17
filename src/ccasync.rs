pub mod rt_async_std {

    use super::*;

    use async_std::io::{ReadExt, WriteExt};
    use async_std::net::TcpStream;
    use async_std::task;

    async fn drop_handle(h: task::JoinHandle<()>) {
        h.await;
    }

    async fn stdout_write(buf: Vec<u8>) {
        async_std::io::stdout().write_all(&buf).await.unwrap();
    }

    ccasync!();

    pub fn start(n: usize, m: usize) {
        task::block_on(send(n, m));
    }
}

pub mod rt_tokio {

    use super::*;

    use std::io::Write;

    use tokio::io::AsyncReadExt;
    use tokio::net::TcpStream;
    use tokio::runtime::Runtime;
    use tokio::task;

    async fn drop_handle(h: task::JoinHandle<()>) {
        h.await.unwrap();
    }

    async fn stdout_write(buf: Vec<u8>) {
        task::spawn_blocking(move || {
            std::io::stdout().write_all(&buf).unwrap();
        });
    }

    ccasync!();

    pub fn start(n: usize, m: usize) {
        let rt = Runtime::new().unwrap();
        rt.block_on(send(n, m));
    }
}

macro_rules! ccasync {
    () => {
        async fn get_count() -> Vec<u8> {
            let mut stream = TcpStream::connect("127.0.0.1:10123").await.unwrap();
            let mut buf = Vec::with_capacity(22);
            stream.read_to_end(&mut buf).await.unwrap();
            drop(stream);
            buf
        }

        async fn send(n: usize, m: usize) {
            let mut handles = Vec::with_capacity(m);
            for _ in 0..n {
                let h = task::spawn(async {
                    let mut buf = get_count().await;
                    let nbuf = buf.len();
                    buf[nbuf - 2] = b'\n';
                    let _ = buf.pop();
                    stdout_write(buf).await;
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
