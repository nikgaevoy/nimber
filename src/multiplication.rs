// Nimber multiplication
// The algorithm is similar to Karatsuba multiplication algorithm
// Some ideas were taken from the C++ implementation by David Eppstein
// (https://www.ics.uci.edu/~eppstein/numth/)

use super::Nimber;
use std::ops::{
    AddAssign, BitAnd, BitOr, BitOrAssign, Div, DivAssign, Mul, MulAssign, Shl, Shr, Sub,
};

type Smallest = u8;
type Shift = usize;
type Level = u8;

// assuming that 1 << 1 << lvl always fits in T
#[inline]
fn high_part<'a, T>(a: &'a Nimber<T>, lvl: Level) -> Nimber<T>
where
    &'a Nimber<T>: Shr<Shift, Output = Nimber<T>>,
{
    a >> ((1 as Shift) << lvl)
}

// assuming that 1 << 1 << lvl always fits in T
#[inline]
fn low_part<'a, T: From<Smallest>>(a: &'a Nimber<T>, lvl: Level) -> Nimber<T>
where
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: BitAnd<&'a Nimber<T>, Output = Nimber<T>>,
{
    Nimber::from((T::from(1 as Smallest) << ((1 as Shift) << lvl)) - T::from(1 as Smallest)) & a
}

// assuming that 1 << 1 << lvl always fits in T, even if a = 0
#[inline]
fn combine<'a, 'b, T>(high: &'a Nimber<T>, low: &'b Nimber<T>, lvl: Level) -> Nimber<T>
    where
        &'a Nimber<T>: Shl<Shift, Output=Nimber<T>>,
        Nimber<T>: BitOr<&'b Nimber<T>, Output=Nimber<T>>,
{
    (high << ((1 as Shift) << lvl)) | low
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

// multiply by 1 << ((1 << lvl) - 1)
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

    let ansh = nimber_mul_fermat::<T>(&asum, lvl);
    let ansl = nimber_mul_fermat::<T>(&nimber_mul_fermat::<T>(&ah, lvl), lvl);

    combine::<T>(&ansh, &ansl, lvl)
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

    let mut ansh = nimber_mul_nimber::<T>(&asum, &bsum, lvl);
    ansh += &low_mul;
    let mut ansl = nimber_mul_fermat::<T>(&nimber_mul_nimber::<T>(&ah, &bh, lvl), lvl);
    ansl += &low_mul;

    combine::<T>(&ansh, &ansl, lvl)
}

/// Multiplication of nimbers
///
/// The complexity is *O*(*n*^(log_2 3) \* log *n*)
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

    #[inline]
    fn mul(self, rhs: &'b Nimber<T>) -> Self::Output {
        nimber_mul_nimber::<T>(self, rhs, level::<T>(&(self | rhs)))
    }
}

nimber_ref_binop!(impl Mul, mul);
nimber_ref_binop_assign!(impl MulAssign, mul_assign use Mul, mul);

fn nimber_square<T: From<Smallest>>(a: &Nimber<T>, lvl: Level) -> Nimber<T>
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
    Nimber<T>: AddAssign<Nimber<T>>,
{
    if lvl == 0 {
        return a.clone();
    }

    let lvl = lvl - 1;

    let mut ah = high_part::<T>(&a, lvl);
    let mut al = low_part::<T>(&a, lvl);

    ah = nimber_square::<T>(&ah, lvl);
    al = nimber_square::<T>(&al, lvl);

    al += nimber_mul_fermat::<T>(&ah, lvl);

    combine::<T>(&ah, &al, lvl)
}

fn nimber_sqrt<T: From<Smallest>>(a: &Nimber<T>, lvl: Level) -> Nimber<T>
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
    Nimber<T>: AddAssign<Nimber<T>>,
{
    if lvl == 0 {
        return a.clone();
    }

    let lvl = lvl - 1;

    let mut ah = high_part::<T>(&a, lvl);
    let mut al = low_part::<T>(&a, lvl);

    al += &nimber_mul_fermat::<T>(&ah, lvl);

    al = nimber_sqrt::<T>(&al, lvl);
    ah = nimber_sqrt::<T>(&ah, lvl);

    combine::<T>(&ah, &al, lvl)
}

fn nimber_inverse<T: From<Smallest>>(a: &Nimber<T>, lvl: Level) -> Nimber<T>
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

    // nimber_mul_nimber
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,

    // impl
    Nimber<T>: AddAssign<Nimber<T>>,
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
{
    if lvl == 0 {
        return a.clone();
    }

    let lvl = lvl - 1;

    let ah = high_part::<T>(&a, lvl);
    let al = low_part::<T>(&a, lvl);
    let mut asum = ah.clone();
    asum += &al;
    let asum = asum;

    let mut det = nimber_mul_nimber::<T>(&asum, &al, lvl);
    det += nimber_mul_fermat::<T>(&nimber_square::<T>(&ah, lvl), lvl);
    let det = nimber_inverse::<T>(&det, lvl);

    combine::<T>(
        &nimber_mul_nimber::<T>(&ah, &det, lvl),
        &nimber_mul_nimber::<T>(&asum, &det, lvl),
        lvl,
    )
}

impl<T: From<Smallest>> Nimber<T>
where
    for<'x> &'x Nimber<T>: Shr<Shift, Output = Nimber<T>>,
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: for<'y> BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: for<'y> BitOr<&'y Nimber<T>, Output = Nimber<T>>,
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    Nimber<T>: Clone,
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    Nimber<T>: AddAssign<Nimber<T>>,
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: BitOrAssign<Nimber<T>> + PartialEq,
{
    /// Takes the reciprocal (inverse) of a nimber, `1 / x`.
    ///
    /// The complexity is *O*(*n*^(log_2 3) \* log *n*).
    #[inline]
    pub fn recip(&self) -> Nimber<T> {
        nimber_inverse::<T>(&self, level::<T>(&self))
    }
}

impl<T: From<Smallest>> Nimber<T>
where
    for<'x> &'x Nimber<T>: Shr<Shift, Output = Nimber<T>>,
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: for<'y> BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: for<'y> BitOr<&'y Nimber<T>, Output = Nimber<T>>,
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    Nimber<T>: Clone,
    Nimber<T>: AddAssign<Nimber<T>>,
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: BitOrAssign<Nimber<T>> + PartialEq,
{
    /// Squares a nimber, `x * x`.
    ///
    /// The complexity is *O*(*n*^(log_2 3)), faster than the general multiplication.
    #[inline]
    pub fn square(&self) -> Nimber<T> {
        nimber_square::<T>(&self, level::<T>(&self))
    }

    /// Returns square root of a nimber.
    ///
    /// The complexity is *O*(*n*^(log_2 3)).
    #[inline]
    pub fn sqrt(&self) -> Nimber<T> {
        nimber_sqrt::<T>(&self, level::<T>(&self))
    }
}

impl<'a, 'b, T: From<Smallest>> Div<&'b Nimber<T>> for &'a Nimber<T>
where
    &'a Nimber<T>: Mul<Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shr<Shift, Output = Nimber<T>>,
    T: Shl<Shift, Output = T> + Sub<Output = T>,
    Nimber<T>: for<'y> BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: for<'y> BitOr<&'y Nimber<T>, Output = Nimber<T>>,
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    Nimber<T>: Clone,
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    Nimber<T>: AddAssign<Nimber<T>>,
    Nimber<T>: for<'y> AddAssign<&'y Nimber<T>>,
    for<'x, 'y> &'x Nimber<T>: BitAnd<&'y Nimber<T>, Output = Nimber<T>>,
    for<'x> &'x Nimber<T>: Shl<Shift, Output = Nimber<T>>,
    Nimber<T>: BitOrAssign<Nimber<T>> + PartialEq,
{
    type Output = Nimber<T>;

    /// Division of nimbers.
    /// Algorithm-wise it is an alias to the multiplication by the inverse.
    ///
    /// The complexity is *O*(*n*^(log_2 3) \* log *n*).
    #[inline]
    fn div(self, rhs: &'b Nimber<T>) -> Self::Output {
        self * rhs.recip()
    }
}

nimber_ref_binop!(impl Div, div);
nimber_ref_binop_assign!(impl DivAssign, div_assign use Div, div);
