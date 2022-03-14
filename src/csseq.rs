use std::io::Write;
use std::net::*;

pub mod simple {

    use super::*;

    pub fn start() {
        let mut counter = 0u64;
        let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
        loop {
            let (mut socket, _addr) = listener.accept().unwrap();
            counter += 1;
            write!(socket, "{}\r\n", counter).unwrap();
            socket.flush().unwrap();
            drop(socket);
        }
    }

}

pub mod fast {

    use super::*;

    pub fn start() {
        let listener = TcpListener::bind("127.0.0.1:10123").unwrap();
        let mut buf = [b'\n'; 22];
        buf[20] = b'\r';
        let b = 19;
        let mut p = b;
        buf[p] = b'0';
        loop {
            let (mut socket, _addr) = listener.accept().unwrap();
            let _ = socket.write_all(&buf[p..]).unwrap();
            buf[b] += 1;
            if buf[b] > b'9' {
                buf[b] = b'0';
                for i in (p..b).rev() {
                    buf[i] += 1;
                    if buf[i] <= b'9' {
                        break;
                    }
                    buf[i] = b'0';
                }
                if buf[p] == b'0' {
                    p -= 1;
                    buf[p] = b'1';
                }
            }
            socket.flush().unwrap();
            drop(socket);
        }
    }

}
