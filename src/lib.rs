use std::ops::{Add, BitXor, AddAssign, BitXorAssign};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Nimber<T> {
    x: T,
}

impl<T: Copy> Copy for Nimber<T> {}

impl<T> From<T> for Nimber<T> {
    fn from(x: T) -> Self {
        Self { x }
    }
}

impl<F: BitXor<S>, S> Add<Nimber<S>> for Nimber<F> {
    type Output = Nimber<<F as BitXor<S>>::Output>;

    fn add(self, rhs: Nimber<S>) -> Self::Output {
        Nimber { x: self.x ^ rhs.x }
    }
}

impl<'a, F: 'a, S> Add<Nimber<S>> for &'a Nimber<F>
    where &'a F: BitXor<S> {
    type Output = Nimber<<&'a F as BitXor<S>>::Output>;

    fn add(self, rhs: Nimber<S>) -> Self::Output {
        Nimber { x: &self.x ^ rhs.x }
    }
}

impl<'b, F: BitXor<&'b S>, S> Add<&'b Nimber<S>> for Nimber<F> {
    type Output = Nimber<<F as BitXor<&'b S>>::Output>;

    fn add(self, rhs: &'b Nimber<S>) -> Self::Output {
        Nimber { x: self.x ^ &rhs.x }
    }
}

impl<'a, 'b, F, S> Add<&'b Nimber<S>> for &'a Nimber<F>
    where &'a F: BitXor<&'b S> {
    type Output = Nimber<<&'a F as BitXor<&'b S>>::Output>;

    fn add(self, rhs: &'b Nimber<S>) -> Self::Output {
        Nimber { x: &self.x ^ &rhs.x }
    }
}

impl<F: BitXorAssign<S>, S> AddAssign<Nimber<S>> for Nimber<F> {
    fn add_assign(&mut self, rhs: Nimber<S>) {
        self.x ^= rhs.x;
    }
}

impl<'b, F: BitXorAssign<&'b S>, S> AddAssign<&'b Nimber<S>> for Nimber<F> {
    fn add_assign(&mut self, rhs: &'b Nimber<S>) {
        self.x ^= &rhs.x;
    }
}

use num_bigint::BigUint;

pub type Nim8 = Nimber<u8>;
pub type Nim16 = Nimber<u16>;
pub type Nim32 = Nimber<u32>;
pub type Nim64 = Nimber<u64>;
pub type Nim128 = Nimber<u128>;
pub type BigNim = Nimber<BigUint>;


#[cfg(test)]
mod tests {
    use crate::{Nim8, BigNim};
    use num_bigint::BigUint;

    #[test]
    fn add() {
        for a in 0u8..8 {
            for b in 0u8..8 {
                assert_eq!(Nim8::from(a) + Nim8::from(b), Nim8::from(a ^ b));
                assert_eq!(BigNim::from(BigUint::from(a)) + BigNim::from(BigUint::from(b)),
                           BigNim::from(BigUint::from(a ^ b)));
            }
        }
    }
}
