pub struct Nimber<T> {
    x: T,
}

mod derive;
mod addition;

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
    use crate::Nim8;

    #[cfg(feature = "num-bigint")]
    use crate::BigNim;
    #[cfg(feature = "num-bigint")]
    use num_bigint::BigUint;

    #[test]
    fn add() {
        for a in 0u8..8 {
            for b in 0u8..8 {
                assert_eq!(Nim8::from(a) + Nim8::from(b), Nim8::from(a ^ b));
            }
        }
    }

    #[test]
    #[cfg(feature = "num-bigint")]
    fn bigint_add() {
        for a in 0u8..8 {
            for b in 0u8..8 {
                assert_eq!(BigNim::from(BigUint::from(a)) + BigNim::from(BigUint::from(b)),
                           BigNim::from(BigUint::from(a ^ b)));
            }
        }
    }

    #[test]
    fn sub() {
        for a in 0u8..8 {
            for b in 0u8..8 {
                assert_eq!(Nim8::from(a) - Nim8::from(b), Nim8::from(a) + Nim8::from(b));
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
}
