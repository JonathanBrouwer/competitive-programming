use std::cmp::Ordering;

/// a and b are iterators containing no duplicates and are sorted
/// returns a Vec of isize that contains no duplicates and is sorted
fn merge_uniq(a: impl Iterator<Item=isize>, b: impl Iterator<Item=isize>) -> Vec<isize> {
    let mut result: Vec<isize> = Vec::with_capacity(a.size_hint().0 + b.size_hint().0);
    let mut a = a.peekable();
    let mut b = b.peekable();

    while let (Some(v1), Some(v2)) = (a.peek(), b.peek()) {
        match v1.cmp(v2) {
            Ordering::Less => {
                result.push(a.next().unwrap());
            }
            Ordering::Equal => {
                result.push(a.next().unwrap());
                b.next().unwrap();
            }
            Ordering::Greater => {
                result.push(b.next().unwrap());
            }
        }
    }

    a.for_each(|v| result.push(v));
    b.for_each(|v| result.push(v));

    result
}