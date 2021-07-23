use super::Nimber;

use std::ops::{Add, Sub, BitXor, AddAssign, SubAssign, BitXorAssign};

// Add

impl<F: BitXor<S>, S> Add<Nimber<S>> for Nimber<F> {
    type Output = Nimber<<F as BitXor<S>>::Output>;

    #[inline]
    fn add(self, rhs: Nimber<S>) -> Self::Output {
        Nimber { x: self.x ^ rhs.x }
    }
}

impl<'a, F: 'a, S> Add<Nimber<S>> for &'a Nimber<F>
    where &'a F: BitXor<S> {
    type Output = Nimber<<&'a F as BitXor<S>>::Output>;

    #[inline]
    fn add(self, rhs: Nimber<S>) -> Self::Output {
        Nimber { x: &self.x ^ rhs.x }
    }
}

impl<'b, F: BitXor<&'b S>, S> Add<&'b Nimber<S>> for Nimber<F> {
    type Output = Nimber<<F as BitXor<&'b S>>::Output>;

    #[inline]
    fn add(self, rhs: &'b Nimber<S>) -> Self::Output {
        Nimber { x: self.x ^ &rhs.x }
    }
}

impl<'a, 'b, F, S> Add<&'b Nimber<S>> for &'a Nimber<F>
    where &'a F: BitXor<&'b S> {
    type Output = Nimber<<&'a F as BitXor<&'b S>>::Output>;

    #[inline]
    fn add(self, rhs: &'b Nimber<S>) -> Self::Output {
        Nimber { x: &self.x ^ &rhs.x }
    }
}

impl<F: BitXorAssign<S>, S> AddAssign<Nimber<S>> for Nimber<F> {
    #[inline]
    fn add_assign(&mut self, rhs: Nimber<S>) {
        self.x ^= rhs.x;
    }
}

impl<'b, F: BitXorAssign<&'b S>, S> AddAssign<&'b Nimber<S>> for Nimber<F> {
    #[inline]
    fn add_assign(&mut self, rhs: &'b Nimber<S>) {
        self.x ^= &rhs.x;
    }
}

// Sub

impl<F: BitXor<S>, S> Sub<Nimber<S>> for Nimber<F> {
    type Output = Nimber<<F as BitXor<S>>::Output>;

    #[inline]
    fn sub(self, rhs: Nimber<S>) -> Self::Output {
        Nimber { x: self.x ^ rhs.x }
    }
}

impl<'a, F: 'a, S> Sub<Nimber<S>> for &'a Nimber<F>
    where &'a F: BitXor<S> {
    type Output = Nimber<<&'a F as BitXor<S>>::Output>;

    #[inline]
    fn sub(self, rhs: Nimber<S>) -> Self::Output {
        Nimber { x: &self.x ^ rhs.x }
    }
}

impl<'b, F: BitXor<&'b S>, S> Sub<&'b Nimber<S>> for Nimber<F> {
    type Output = Nimber<<F as BitXor<&'b S>>::Output>;

    #[inline]
    fn sub(self, rhs: &'b Nimber<S>) -> Self::Output {
        Nimber { x: self.x ^ &rhs.x }
    }
}

impl<'a, 'b, F, S> Sub<&'b Nimber<S>> for &'a Nimber<F>
    where &'a F: BitXor<&'b S> {
    type Output = Nimber<<&'a F as BitXor<&'b S>>::Output>;

    #[inline]
    fn sub(self, rhs: &'b Nimber<S>) -> Self::Output {
        Nimber { x: &self.x ^ &rhs.x }
    }
}

impl<F: BitXorAssign<S>, S> SubAssign<Nimber<S>> for Nimber<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Nimber<S>) {
        self.x ^= rhs.x;
    }
}

impl<'b, F: BitXorAssign<&'b S>, S> SubAssign<&'b Nimber<S>> for Nimber<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: &'b Nimber<S>) {
        self.x ^= &rhs.x;
    }
}
