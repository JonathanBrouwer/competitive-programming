use std::ops::Index;

#[derive(Eq, PartialEq, Clone)]
struct OIVec<T>(Vec<T>);

impl<T> Index<usize> for OIVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}