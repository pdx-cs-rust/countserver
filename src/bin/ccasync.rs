use async_std::io;
use async_std::net::*;
use async_std::prelude::*;
use async_std::task;

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
    let mut stdout = io::stdout();
    for h in handles {
        writeln!(stdout, "{}", h.await).await.unwrap();
    }
}

fn main() {
    let n = std::env::args().nth(1).unwrap().parse().unwrap();
    task::block_on(send(n));
}
