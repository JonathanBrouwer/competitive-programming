use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

/// A binary search algorithm that is generic over what is being searched
/// f says whether the guess is higher/equal/lower than the correct value
fn binary_search(mut low: usize, mut high: usize, f: impl Fn(usize) -> Ordering) -> Result<usize, usize> {
    while low <= high {
        let mid = (low + high) / 2;

        match f(mid) {
            Less => low = mid + 1,
            Equal => return Ok(mid),
            Greater => high = mid - 1,
        }
    }

    Err(low)
}