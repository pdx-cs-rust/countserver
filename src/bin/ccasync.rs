#[cfg(feature = "async-std-rt")]
mod async_rt {
    pub use async_std::io::{self, ReadExt, WriteExt};
    pub use async_std::net::TcpStream;
    pub use async_std::task;
}

#[cfg(feature = "tokio-rt")]
mod async_rt {
    pub use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
    pub use tokio::net::TcpStream;
    pub use tokio::task;
    pub use tokio::runtime::Runtime;
}

use async_rt::*;

async fn get_count() -> u64 {
    let mut stream = TcpStream::connect("127.0.0.1:10123").await.unwrap();
    let mut buf = String::with_capacity(22);
    stream.read_to_string(&mut buf).await.unwrap();
    drop(stream);
    buf.trim_end().parse().unwrap()
}

async fn send(n: usize) {
    for _ in 0..n {
        let _status = task::spawn(async {
            let c = get_count().await;
            io::stdout()
                .write_all(format!("{}\n", c).as_bytes())
                .await
                .unwrap();
        }).await;
        #[cfg(feature = "tokio-rt")]
        _status.unwrap();
    }
}

fn main() {
    let n = std::env::args().nth(1).unwrap().parse().unwrap();
    #[cfg(feature = "tokio-rt")] {
        let rt = Runtime::new().unwrap();
        rt.block_on(send(n));
    }
    #[cfg(feature = "async-std-rt")]
    task::block_on(send(n));
}
