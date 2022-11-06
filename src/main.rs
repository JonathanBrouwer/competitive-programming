use std::fmt::Write;
use std::io;
use std::io::{stdin, stdout, BufRead, StdinLock};
use std::marker::PhantomData;
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut scan = Scanner::new();
    let mut print = Printer::new();

    let mut vec: Vec<(usize, usize)> = Vec::with_capacity(200005);

    for _ in 0..scan.next_usize() {
        vec.clear();

        let n = scan.next_usize();
        let s1 = scan.next_line();
        let s2 = scan.next_line();

        //Impossible
        let eq = s1 == s2;
        if !(eq || s1.bytes().zip(s2.bytes()).all(|(b1, b2)| b1 != b2)) {
            writeln!(print, "NO").unwrap();
            continue;
        }

        let mut res = Vec::with_capacity(n + 3);
        for (i, b) in s1.bytes().enumerate() {
            if b == b'1' {
                res.push((i + 1, i + 1));
            }
        }

        if eq ^ (res.len() % 2 == 0) {
            res.push((1, n));
            res.push((1, 1));
            res.push((2, n));
        }

        writeln!(print, "YES").unwrap();
        writeln!(print, "{}", res.len()).unwrap();
        res.iter()
            .for_each(|(x, y)| writeln!(print, "{} {}", x, y).unwrap());
    }
}

struct Scanner {
    reader: StdinLock<'static>,
    buf_str: String,
    buf_iter: SplitAsciiWhitespace<'static>,
}
impl Scanner {
    fn new() -> Self {
        Self {
            reader: stdin().lock(),
            buf_str: String::new(),
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    fn next_usize(&mut self) -> usize {
        self.next()
    }

    fn next_line(&mut self) -> String {
        assert!(self.buf_iter.next() == None);

        let mut line = String::new();
        self.reader
            .read_until(b'\n', unsafe { line.as_mut_vec() })
            .expect("Failed read");
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }

        line
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', unsafe { self.buf_str.as_mut_vec() })
                .expect("Failed read");
            self.buf_iter = unsafe {
                // Safety: We don't use `buf_vec` until this iter is empty.
                std::mem::transmute(self.buf_str.split_ascii_whitespace())
            }
        }
    }

    fn iter<T: std::str::FromStr>(&mut self) -> ScannerIter<T> {
        ScannerIter {
            phantom: Default::default(),
            scanner: self,
        }
    }
}

struct ScannerIter<'a, T: std::str::FromStr> {
    phantom: PhantomData<T>,
    scanner: &'a mut Scanner,
}

impl<'a, T: std::str::FromStr> Iterator for ScannerIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.scanner.next())
    }
}

struct Printer {
    buffer: String,
}
impl Printer {
    fn new() -> Self {
        Self {
            buffer: String::with_capacity(64),
        }
    }

    fn flush(&mut self) {
        io::Write::write_all(&mut stdout(), self.buffer.as_bytes()).unwrap();
        self.buffer.clear();
        self.buffer.reserve_exact(self.buffer.capacity());
    }
}
impl Write for Printer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.buffer.capacity() - self.buffer.len() < s.len() {
            self.flush();
        }
        self.buffer.write_str(s)
    }
}
impl Drop for Printer {
    fn drop(&mut self) {
        self.flush()
    }
}
