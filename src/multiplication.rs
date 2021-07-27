// Nimber multiplication
// The algorithm is similar to Karatsuba multiplication algorithm
// Some ideas were taken from the C++ implementation by David Eppstein
// (https://www.ics.uci.edu/~eppstein/numth/)

use super::Nimber;
use std::ops::{AddAssign, BitAnd, BitOr, BitOrAssign, Mul, MulAssign, Shl, Shr, Sub};

type Smallest = u8;
type Shift = usize;
type Level = u8;

// assuming that 1 << 1 << lvl always fits in T
fn high_part<'a, T>(a: &'a Nimber<T>, lvl: Level) -> Nimber<T>
where
    &'a Nimber<T>: Shr<Shift, Output = Nimber<T>>,
{
    a >> ((1 as Shift) << lvl)
}

// assuming that 1 << 1 << lvl always fits in T
fn low_part<'a, T: From<Smallest>>(a: &'a Nimber<T>, lvl: Level) -> Nimber<T>
where
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: BitAnd<&'a Nimber<T>, Output = Nimber<T>>,
{
    Nimber::from((T::from(1 as Smallest) << ((1 as Shift) << lvl)) - T::from(1 as Smallest)) & a
}

// assuming that 1 << 1 << lvl always fits in T, even if a = 0
fn combine<'a, 'b, T>(a: &'a Nimber<T>, b: &'b Nimber<T>, lvl: Level) -> Nimber<T>
where
    &'a Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: BitOr<&'b Nimber<T>, Output = Nimber<T>>,
{
    (a << ((1 as Shift) << lvl)) | b
}

// finds smallest level at which high_part is 0
// implementation differs from the naive (with computing high_part in a loop),
// because the naive version overflows at shl
fn level<'a, T: From<Smallest>>(a: &'a Nimber<T>) -> Level
where
    &'a Nimber<T>: for<'y> BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: BitOrAssign<Nimber<T>> + PartialEq,
{
    let mut lvl = 0;
    let mut low_mask = Nimber::from(T::from(1 as Smallest));

    while a & &low_mask != *a {
        low_mask |= &low_mask << ((1 as Shift) << lvl);
        lvl += 1;
    }

    lvl
}

// multiply by 1 << 1 << (lvl - 1)
fn nimber_mul_fermat<'a, T: From<Smallest>>(a: &'a Nimber<T>, lvl: Level) -> Nimber<T>
where
    // high_part
    for<'x> &'x Nimber<T>: Shr<Shift, Output = Nimber<T>>,

    // low_part
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: for<'y> BitAnd<&'y Nimber<T>, Output = Nimber<T>>,

    // combine
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: for<'y> BitOr<&'y Nimber<T>, Output = Nimber<T>>,

    // impl
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    Nimber<T>: Clone,
{
    if lvl == 0 {
        return a.clone();
    }

    let lvl = lvl - 1;

    let ah = high_part::<T>(a, lvl);
    let mut al = low_part::<T>(a, lvl);

    al += &ah;

    let asum = al;

    let low = nimber_mul_fermat::<T>(&asum, lvl);
    let high = nimber_mul_fermat::<T>(&nimber_mul_fermat::<T>(&ah, lvl), lvl);

    combine::<T>(&low, &high, lvl)
}

fn nimber_mul_nimber<'a, 'b, T: From<Smallest>>(
    a: &'a Nimber<T>,
    b: &'b Nimber<T>,
    lvl: Level,
) -> Nimber<T>
where
    // high_part
    for<'x> &'x Nimber<T>: Shr<Shift, Output = Nimber<T>>,

    // low_part
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: for<'y> BitAnd<&'y Nimber<T>, Output = Nimber<T>>,

    // combine
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: for<'y> BitOr<&'y Nimber<T>, Output = Nimber<T>>,

    // nimber_mul_fermat
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    Nimber<T>: Clone,

    // impl
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
{
    if lvl == 0 {
        return a & b;
    }

    let lvl = lvl - 1;

    let ah = high_part::<T>(a, lvl);
    let mut al = low_part::<T>(a, lvl);
    let bh = high_part::<T>(b, lvl);
    let mut bl = low_part::<T>(b, lvl);

    let low_mul = nimber_mul_nimber::<T>(&al, &bl, lvl);

    al += &ah;
    let asum = al;
    bl += &bh;
    let bsum = bl;

    let mut ansl = nimber_mul_nimber::<T>(&asum, &bsum, lvl);
    ansl += &low_mul;
    let mut ansh = nimber_mul_fermat::<T>(&nimber_mul_nimber::<T>(&ah, &bh, lvl), lvl);
    ansh += &low_mul;

    combine::<T>(&ansl, &ansh, lvl)
}

/// Multiplication of nimbers
///
/// The complexity is O(n^(log_2 3) log n)
impl<'a, 'b, T: PartialEq + From<Smallest>> Mul<&'b Nimber<T>> for &'a Nimber<T>
where
    for<'x> &'x Nimber<T>: Shr<Shift, Output = Nimber<T>>,
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: for<'y> BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: for<'y> BitOr<&'y Nimber<T>, Output = Nimber<T>>,
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    Nimber<T>: Clone,
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: BitOrAssign<Nimber<T>> + PartialEq,
    &'a Nimber<T>: BitOr<&'b Nimber<T>, Output = Nimber<T>>,
{
    type Output = Nimber<T>;

    fn mul(self, rhs: &'b Nimber<T>) -> Self::Output {
        nimber_mul_nimber::<T>(self, rhs, level::<T>(&(self | rhs)))
    }
}

nimber_ref_binop!(impl Mul, mul);
nimber_ref_binop_assign!(impl MulAssign, mul_assign use Mul, mul);
