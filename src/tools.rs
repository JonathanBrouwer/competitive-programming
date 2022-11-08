fn period(v: &Vec<u8>) -> usize {
    let n = v.len();
    'l: for period in 1..=n/2 {
        if n % period != 0 { continue }
        for i in period..n {
            if v[i] != v[i - period] {
                continue 'l;
            }
        }
        return period;
    }
    return n
}