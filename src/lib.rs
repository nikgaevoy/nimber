pub struct Nimber<T> {
    x: T,
}

#[macro_use]
mod macros;

mod derive;
mod addition;
mod multiplication;

pub type Nim8 = Nimber<u8>;
pub type Nim16 = Nimber<u16>;
pub type Nim32 = Nimber<u32>;
pub type Nim64 = Nimber<u64>;
pub type Nim128 = Nimber<u128>;

#[cfg(feature = "num-bigint")]
use num_bigint::BigUint;

#[cfg(feature = "num-bigint")]
pub type BigNim = Nimber<BigUint>;


#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(feature = "num-bigint")]
    use crate::BigNim;
    #[cfg(feature = "num-bigint")]
    use num_bigint::BigUint;

    #[allow(dead_code)]
    const MUL_TABLE: [[u32; 16]; 16] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [0, 2, 3, 1, 8, 10, 11, 9, 12, 14, 15, 13, 4, 6, 7, 5],
        [0, 3, 1, 2, 12, 15, 13, 14, 4, 7, 5, 6, 8, 11, 9, 10],
        [0, 4, 8, 12, 6, 2, 14, 10, 11, 15, 3, 7, 13, 9, 5, 1],
        [0, 5, 10, 15, 2, 7, 8, 13, 3, 6, 9, 12, 1, 4, 11, 14],
        [0, 6, 11, 13, 14, 8, 5, 3, 7, 1, 12, 10, 9, 15, 2, 4],
        [0, 7, 9, 14, 10, 13, 3, 4, 15, 8, 6, 1, 5, 2, 12, 11],
        [0, 8, 12, 4, 11, 3, 7, 15, 13, 5, 1, 9, 6, 14, 10, 2],
        [0, 9, 14, 7, 15, 6, 1, 8, 5, 12, 11, 2, 10, 3, 4, 13],
        [0, 10, 15, 5, 3, 9, 12, 6, 1, 11, 14, 4, 2, 8, 13, 7],
        [0, 11, 13, 6, 7, 12, 10, 1, 9, 2, 4, 15, 14, 5, 3, 8],
        [0, 12, 4, 8, 13, 1, 9, 5, 6, 10, 2, 14, 11, 7, 15, 3],
        [0, 13, 6, 11, 9, 4, 15, 2, 14, 3, 8, 5, 7, 10, 1, 12],
        [0, 14, 7, 9, 5, 11, 2, 12, 10, 4, 13, 3, 15, 1, 8, 6],
        [0, 15, 5, 10, 1, 14, 4, 11, 2, 13, 7, 8, 3, 12, 6, 9]
    ];


    #[test]
    fn add() {
        for a in u8::MIN..u8::MAX {
            for b in u8::MIN..u8::MAX {
                assert_eq!(Nimber::from(a) + Nimber::from(b), Nimber::from(a ^ b));
            }
        }
    }

    #[test]
    fn assign_same() {
        for a in u8::MIN..u8::MAX {
            for b in u8::MIN..u8::MAX {
                let mut sa = Nimber::from(a);
                sa += Nimber::from(b);
                assert_eq!(sa, Nimber::from(a) + Nimber::from(b));

                let mut sa = Nimber::from(a);
                sa *= Nimber::from(b);
                assert_eq!(sa, Nimber::from(a) * Nimber::from(b));
            }
        }
    }

    #[test]
    #[cfg(feature = "num-bigint")]
    fn bigint_add() {
        for a in u8::MIN..u8::MAX {
            for b in u8::MIN..u8::MAX {
                assert_eq!(BigNim::from(BigUint::from(a)) + BigNim::from(BigUint::from(b)),
                           BigNim::from(BigUint::from(a ^ b)));
            }
        }
    }

    #[test]
    fn sub() {
        for a in u8::MIN..u8::MAX {
            for b in u8::MIN..u8::MAX {
                assert_eq!(Nimber::from(a) - Nimber::from(b), Nimber::from(a) + Nimber::from(b));
            }
        }
    }

    #[test]
    fn ord() {
        let two = Nim8::from(2);
        let three = Nim8::from(3);
        let four = Nim8::from(4);

        assert!(two < three);
        assert!(two <= three);
        assert!(three <= three);
        assert!(three >= three);
        assert_eq!(three, three);
        assert!(four > three);
        assert!(four >= three);
        assert_ne!(two, three);
    }

    #[test]
    fn mul() {
        for a in 0..MUL_TABLE.len() {
            for b in 0..MUL_TABLE[a].len() {
                assert_eq!(Nimber::from(a) * Nimber::from(b),
                           Nimber::from(MUL_TABLE[a][b] as usize));
            }
        }
    }

    #[test]
    fn mul_same() {
        for a in 0..MUL_TABLE.len() {
            for b in 0..MUL_TABLE[a].len() {
                let m8 = Nim8::from(a as u8) * Nim8::from(b as u8);
                let m16 = Nim16::from(a as u16) * Nim16::from(b as u16);
                let m32 = Nim32::from(a as u32) * Nim32::from(b as u32);
                let m64 = Nim64::from(a as u64) * Nim64::from(b as u64);
                let m128 = Nim128::from(a as u128) * Nim128::from(b as u128);
                let musize = Nimber::from(a) * Nimber::from(b);

                assert_eq!(m8.x as u16, m16.x);
                assert_eq!(m16.x as u32, m32.x);
                assert_eq!(m32.x as u64, m64.x);
                assert_eq!(m64.x as u128, m128.x);
                assert_eq!(musize.x as u128, m128.x);
            }
        }
    }

    #[test]
    #[cfg(feature = "num-bigint")]
    fn bigint_mul_same() {
        for a in 0..MUL_TABLE.len() {
            for b in 0..MUL_TABLE[a].len() {
                assert_eq!((Nimber::from(BigUint::from(a)) * Nimber::from(BigUint::from(b))).x,
                           BigUint::from((Nimber::from(a) * Nimber::from(b)).x));
            }
        }
    }

    #[test]
    fn neg_not() {
        for a in u8::MIN..u8::MAX {
            assert_eq!(!Nimber::from(a), Nimber::from(!a));
            assert_eq!(-Nimber::from(a), Nimber::from(a));
        }
    }
}
