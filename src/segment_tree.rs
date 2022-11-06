use std::collections::Bound;
use std::mem;
use std::mem::MaybeUninit;
use std::ops::RangeBounds;

// https://codeforces.com/blog/entry/18051
struct SegmentTree<N: Default, F: Fn(&N, &N) -> N> {
    buf: Box<[N]>,
    f: F,
}

impl<N: Default, F: Fn(&N, &N) -> N> SegmentTree<N, F> {
    pub fn build(from: impl ExactSizeIterator<Item=N>, f: F) -> Self {
        let n = from.len();
        let mut buf = (0..2*n).map(|_| MaybeUninit::uninit()).collect::<Vec<_>>().into_boxed_slice();
        for (i, v) in from.enumerate() {
            buf[n+i] = MaybeUninit::new(v)
        }
        for i in (1..=n-1).rev() {
            buf[i] = MaybeUninit::new(f(unsafe { buf[i<<1].assume_init_ref() }, unsafe { buf[i<<1|1].assume_init_ref() }));
        }
        Self {
            buf: unsafe { mem::transmute(buf) }, f
        }
    }

    pub fn query<R: RangeBounds<usize>>(&self, index: R) -> N {
        let n = self.buf.len() / 2;
        let l = match index.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
            Bound::Unbounded => 0,
        };
        let r = match index.end_bound() {
            Bound::Included(i) => *i + 1,
            Bound::Excluded(i) => *i,
            Bound::Unbounded => n,
        };
        assert!(l < r && r <= n, "Range must be in bounds and non-empty.");

        let mut resl = N::default();
        let mut resr = N::default();
        let mut l = l + n;
        let mut r = r + n;
        while l < r {
            if l & 1 == 1 {
                resl = (self.f)(&resl, &self.buf[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                resr = (self.f)(&self.buf[r], &resr);
            }
            l >>= 1;
            r >>= 1;
        }


        (self.f)(&resl, &resr)
    }

    pub fn update(&mut self, mut p: usize, u: impl FnOnce(&mut N) -> ()) {
        let n = self.buf.len() / 2;
        p += n;

        u(&mut self.buf[p]);

        while { p >>= 1; p > 0 } {
            self.buf[p] = (self.f)(&self.buf[p<<1], &self.buf[p<<1|1]);
        }
    }

    pub fn top(&self) -> &N {
        &self.buf[1]
    }
}
