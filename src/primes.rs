struct PrimeIterator {
    primes: Vec<usize>
}

fn primes() -> PrimeIterator {
    PrimeIterator{ primes: Vec::new() }
}

impl Iterator for PrimeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.primes.last().map(|x| *x + 1).unwrap_or(2).. {
            if self.primes.iter().all(|prime| i % prime != 0) {
                self.primes.push(i);
                return Some(i)
            }
        }
        unreachable!()
    }
}

fn firstfac(x: usize) -> usize {
    if x % 2 == 0 {
        return 2;
    };
    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    x
}

pub fn is_prime(n: usize) -> bool {
    if n <= 1 { return false }
    firstfac(n) == n
}

fn factor_count(mut n: usize) -> usize {
    let mut count = 0;
    for d in 2..=((n as f64).sqrt() as usize) {
        while n % d == 0 {
            n /= d;
            count += 1;
        }
        if n == 1 { break }
    }
    if n > 1 {
        count += 1;
    }
    count
}

fn divisors(n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = vec![1];
    for (d, dc) in factors(n) {
        for i in 0..result.len() {
            let mut v = result[i];
            for _ in 1..=dc {
                v *= d;
                result.push(v);
            }
        }
    }
    result
}

fn factors1(mut n: usize) -> Vec<(usize, usize)> {
    let mut factors = Vec::new();
    for d in 2..=((n as f64).sqrt() as usize) {
        let mut count = 0;
        while n % d == 0 {
            n /= d;
            count += 1;
        }
        if count > 0 {
            factors.push((d, count));
        }
        if n == 1 { break }
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

fn factors2(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    for d in 2..=((n as f64).sqrt() as usize) {
        while n % d == 0 {
            n /= d;
            factors.push(d);
        }
        if n == 1 { break }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}
