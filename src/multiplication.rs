use super::Nimber;
use std::ops::{Mul, Shr, Shl, Sub, BitAnd, BitOr, AddAssign};


type Smallest = u8;
type Shift = usize;
type Level = u8;


fn high_part<'a, T>(a: &'a Nimber<T>, lvl: Level) -> Nimber<T>
    where &'a T: Shr<Shift, Output=T> {
    Nimber { x: &a.x >> ((1 as Shift) << lvl) }
}


fn low_part<T: From<Smallest>>(a: &Nimber<T>, lvl: Level) -> Nimber<T>
    where T: for<'bo> BitAnd<&'bo T, Output=T> + Shl<Shift, Output=T> + Sub<Output=T> {
    Nimber { x: ((T::from(1 as Smallest) << ((1 as Shift) << lvl)) - T::from(1 as Smallest)) & &a.x }
}

fn combine<'a, 'b, T>(a: &'a Nimber<T>, b: &'b Nimber<T>, lvl: Level) -> Nimber<T>
    where T: for<'c> BitOr<&'c T, Output=T>, &'a T: Shl<Shift, Output=T> {
    Nimber { x: (&a.x << ((1 as Shift) << lvl)) | &b.x }
}


fn level<'a, T>(a: &'a Nimber<T>) -> Level
    where &'a T: Shr<Shift, Output=T>,
          T: PartialEq + From<Smallest> {
    let mut lvl = 0;

    while high_part(&a, lvl).x != T::from(0 as Smallest) {
        lvl += 1;
    }

    lvl
}


fn half_mul<'a, T: Clone + From<Smallest>>(a: &'a Nimber<T>, lvl: Level) -> Nimber<T>
    where T: for<'b> BitAnd<&'b T, Output=T> + for<'b> BitOr<&'b T, Output=T>,
          T: Shl<Shift, Output=T> + Sub<Output=T>,
          for<'b> &'b T: Shr<Shift, Output=T> + Shl<Shift, Output=T>,
          Nimber<T>: for<'b> AddAssign<&'b Nimber<T>>
{
    if lvl == 0 {
        return a.clone();
    }

    let lvl = lvl - 1;

    let ah = high_part(a, lvl);
    let mut al = low_part(a, lvl);

    al += &ah;

    let asum = al;

    let low = half_mul(&asum, lvl);
    let high = half_mul(&half_mul(&ah, lvl), lvl);

    combine(&low, &high, lvl)
}


fn nim_mul<T: Clone + From<Smallest>>(a: &Nimber<T>, b: &Nimber<T>, lvl: Level) -> Nimber<T>
    where T: for<'b> BitAnd<&'b T, Output=T> + for<'b> BitOr<&'b T, Output=T>,
          T: Shl<Shift, Output=T> + Sub<Output=T>,
          for<'b> &'b T: BitAnd<&'b T, Output=T> + Shr<Shift, Output=T> + Shl<Shift, Output=T>,
          Nimber<T>: for<'b> AddAssign<&'b Nimber<T>>
{
    if lvl == 0 {
        return Nimber { x: (&a.x) & (&b.x) };
    }

    let lvl = lvl - 1;

    let ah = high_part(&a, lvl);
    let mut al = low_part(&a, lvl);
    let bh = high_part(&b, lvl);
    let mut bl = low_part(&b, lvl);

    let low_mult = nim_mul(&al, &bl, lvl);

    al += &ah;
    let asum = al;
    bl += &bh;
    let bsum = bl;

    let mut ansl = nim_mul(&asum, &bsum, lvl);
    ansl += &low_mult;
    let mut ansh = half_mul(&nim_mul(&ah, &bh, lvl), lvl);
    ansh += &low_mult;

    combine(&ansl, &ansh, lvl)
}

impl<T: Clone + PartialEq + From<Smallest>> Mul for Nimber<T>
    where T: for<'b> BitAnd<&'b T, Output=T> + for<'b> BitOr<&'b T, Output=T>,
          T: Shl<Shift, Output=T> + Sub<Output=T>,
          for<'b> &'b T: BitAnd<&'b T, Output=T> + BitOr<&'b T, Output=T>,
          for<'b> &'b T: Shr<Shift, Output=T> + Shl<Shift, Output=T>,
          Nimber<T>: for<'b> AddAssign<&'b Nimber<T>>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        nim_mul(&self, &rhs, level(&(Nimber::<T> { x: &self.x | &rhs.x })))
    }
}
