use super::Nimber;

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

nimber_nimber_forward_binop!(impl Add, add as BitXor, bitxor);
nimber_nimber_forward_binop_assign!(impl AddAssign, add_assign as BitXorAssign, bitxor_assign);

nimber_nimber_forward_binop!(impl Sub, sub as BitXor, bitxor);
nimber_nimber_forward_binop_assign!(impl SubAssign, sub_assign as BitXorAssign, bitxor_assign);

nimber_nimber_forward_binop!(impl BitXor, bitxor);
nimber_nimber_forward_binop_assign!(impl BitXorAssign, bitxor_assign);

nimber_nimber_forward_binop!(impl BitOr, bitor);
nimber_nimber_forward_binop_assign!(impl BitOrAssign, bitor_assign);

nimber_nimber_forward_binop!(impl BitAnd, bitand);
nimber_nimber_forward_binop_assign!(impl BitAndAssign, bitand_assign);

nimber_val_forward_binop!(impl Shr, shr);
nimber_val_forward_binop_assign!(impl ShrAssign, shr_assign);

nimber_val_forward_binop!(impl Shl, shl);
nimber_val_forward_binop_assign!(impl ShlAssign, shl_assign);

impl<T> Neg for Nimber<T> {
    type Output = Nimber<T>;

    fn neg(self) -> Self::Output {
        self
    }
}

impl<T> Neg for &Nimber<T>
where
    Nimber<T>: Clone,
{
    type Output = Nimber<T>;

    fn neg(self) -> Self::Output {
        self.clone()
    }
}

impl<T: Not<Output = T>> Not for Nimber<T> {
    type Output = Nimber<T>;

    fn not(self) -> Self::Output {
        Nimber::from(!self.x)
    }
}

impl<'a, T> Not for &'a Nimber<T>
where
    &'a T: Not<Output = T>,
{
    type Output = Nimber<T>;

    fn not(self) -> Self::Output {
        Nimber::from(!&self.x)
    }
}
