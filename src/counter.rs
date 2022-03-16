pub struct Counter {
    p: usize,
    bytes: [u8; 22],
}

impl Default for Counter {
    fn default() -> Self {
        let mut bytes = [b'\n'; 22];
        bytes[19] = b'0';
        Self { p: 19, bytes }
    }
}

impl Counter {
    pub fn value(&self) -> &[u8] {
        &self.bytes[self.p..]
    }

    pub fn inc(&mut self) {
        let b = 19;
        let mut p = self.p;
        let buf = &mut self.bytes;
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
        self.p = p;
    }
}
