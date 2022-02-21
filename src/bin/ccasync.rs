#[cfg(feature = "async-std-rt")]
mod async_rt {
    pub use async_std::io::{self, ReadExt};
    pub use async_std::net::TcpStream;
    pub use async_std::task;
}

#[cfg(feature = "tokio-rt")]
mod async_rt {
    pub use tokio::io::{self, AsyncReadExt};
    pub use tokio::net::TcpStream;
    pub use tokio::task;
    pub use tokio::runtime::Runtime;
}

use async_rt::*;

async fn get_count() -> u64 {
    let mut stream = TcpStream::connect("127.0.0.1:10123").await.unwrap();
    let mut buf = String::with_capacity(22);
    stream.read_to_string(&mut buf).await.unwrap();
    buf.trim_end().parse().unwrap()
}


async fn send(n: usize) {
    let mut handles = Vec::with_capacity(n);
    for _ in 0..n {
        handles.push(task::spawn(get_count()));
    }
    for h in handles {
        println!("{:?}", h.await);
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
