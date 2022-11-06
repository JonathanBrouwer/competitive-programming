use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
struct M<const P: usize>(usize);

impl<const P: usize> Add for M<P> {
    type Output = M<P>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let mut v = self.0 + rhs.0;
        if v >= P {
            v -= P
        }
        M(v)
    }
}

impl<const P: usize> Sub for M<P> {
    type Output = M<P>;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = self.0.wrapping_sub(rhs.0);
        if v >= P {
            v = v.wrapping_add(P)
        }
        M(v)
    }
}

impl<const P: usize> Mul for M<P> {
    type Output = M<P>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        M(((self.0 as u128 * rhs.0 as u128) % P as u128) as usize)
    }
}


impl<const P: usize> Display for M<P> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<const P: usize> M<P> {
    #[inline(always)]
    pub fn pow(mut self, n: Self) -> Self {
        let mut n = n.0;
        let mut res: Self = Self(1);
        while n > 0 {
            if n % 2 == 1 { res = res * self }
            self = self * self;
            n /= 2;
        }

        res
    }

    #[inline(always)]
    pub fn inv(self) -> Self {
        self.pow(M(P - 2))
    }
}