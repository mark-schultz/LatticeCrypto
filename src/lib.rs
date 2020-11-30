//! The name stands for "Finite Rank Commutative Ring", and it serves as the base ring
//! that arithmetic within this package will be done within. Mathematically, a rank n
//! commutative ring (over an implicit base ring S, such as Z/QZ) is a ring R that is
//! a rank n S-module. A great example of such rings are polynomial rings R[x]/(f(x)),
//! which are (assuming f(x) is irreducible over R) rank n commutative rings over R.
//!
//! I am trying to design things such that implementing RLWE and MLWE is as simple as
//! possible later. I believe the abstraction of Finite Rank Commutative Rings will be
//! useful for this. Important subclasses of these rings are:
//!     * Finite commutative rings (which are rank 1 over themselves)
//!     * Quotients of Polynomial Rings
//!
//! It is possible I could somewhat stretch the (mathematical) definition of the above
//! to include CRT-friendly rings into such an interface, as R \cong R_1 x ... x R_n
//! can implement From<[u32; n]>, assuming each R_i is small enough. I am not yet sure
//! if this is something I want to do.

#![feature(min_const_generics)]

use alga::general::*;
use num_traits::identities::{One, Zero};
use std::convert::From;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[macro_use]
extern crate alga_derive;

pub trait FinRankCRing<const RANK: usize> {}

impl<T: RingCommutative + From<[u32; RANK]>, const RANK: usize> FinRankCRing<RANK> for T {}

pub mod modular {
    use super::*;

    /// The ring Z/QZ for arbitrary [1^] Q.
    /// Elements are represented as integers in [0, ..., Q)
    ///
    /// This ring implements From<[u32; 1]> rather than From<u32> to better match
    /// higher-rank rings I intend to implement later, which will implement From<[u32; N]> for constant
    /// N.
    ///
    /// [1]: If Q is too large one has to convert from u32's to u64's for addition/multiplication.
    /// "Too large" is determined at compile time, so this should not have a runtime impact if
    /// Q < 2^31 (for addition) or Q < 2^16 (for multiplication).

    #[derive(Clone, Copy, PartialEq, Debug, Default, Alga)]
    #[alga_traits(RingCommutative(Additive, Multiplicative))]
    pub struct Modular<const Q: u32>(u32);

    impl<const Q: u32> From<[u32; 1]> for Modular<Q> {
        fn from(x: [u32; 1]) -> Self {
            Modular(x[0] % Q)
        }
    }

    macro_rules! checked_opp {
        ($func:ident, $bound:ident, $checked_func:ident) => {
            impl<const Q: u32> $bound<Modular<Q>> for Modular<Q> {
                type Output = Modular<Q>;
                fn $func(self, other: Self) -> Self::Output {
                    if let None = u32::$checked_func(Q, Q) {
                        // Less efficient case if func can overflow
                        // As Q is const this is compiled away if not needed
                        let x: u64 = self.0.into();
                        let y: u64 = other.0.into();
                        let modulus: u64 = Q.into();
                        Modular::from([(u64::$func(x, y) % modulus) as u32])
                    } else {
                        Modular::from([u32::$func(self.0, other.0)])
                    }
                }
            }
        };
    }
    checked_opp!(add, Add, checked_add);
    checked_opp!(mul, Mul, checked_mul);
    impl<const Q: u32> Neg for Modular<Q> {
        type Output = Modular<Q>;
        fn neg(self) -> Self::Output {
            Modular::from([Q - self.0])
        }
    }

    impl<const Q: u32> Sub<Modular<Q>> for Modular<Q> {
        type Output = Modular<Q>;
        fn sub(self, other: Self) -> Self::Output {
            Modular::from([self.0 + other.neg().0])
        }
    }

    macro_rules! op_assign {
        ($func:ident, $bound:ident, $method:ident) => {
            impl<const Q: u32> $bound<Modular<Q>> for Modular<Q> {
                fn $func(&mut self, other: Self) {
                    *self = self.$method(other)
                }
            }
        };
    }
    op_assign!(add_assign, AddAssign, add);
    op_assign!(mul_assign, MulAssign, mul);
    op_assign!(sub_assign, SubAssign, sub);

    impl<const Q: u32> Zero for Modular<Q> {
        fn zero() -> Self {
            Modular(0)
        }
        fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }

    impl<const Q: u32> One for Modular<Q> {
        fn one() -> Self {
            Modular(1)
        }
    }

    impl<const Q: u32> Identity<Additive> for Modular<Q> {
        fn identity() -> Self {
            Self::zero()
        }
    }

    impl<const Q: u32> Identity<Multiplicative> for Modular<Q> {
        fn identity() -> Self {
            Self::one()
        }
    }

    impl<const Q: u32> AbstractMagma<Additive> for Modular<Q> {
        fn operate(&self, other: &Self) -> Self {
            *self + *other
        }
    }

    impl<const Q: u32> TwoSidedInverse<Additive> for Modular<Q> {
        fn two_sided_inverse(&self) -> Self {
            Self::zero() - *self
        }
    }

    impl<const Q: u32> AbstractMagma<Multiplicative> for Modular<Q> {
        fn operate(&self, other: &Self) -> Self {
            *self * *other
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_add() {
            const Q: u32 = 13;
            let x = Modular::<Q>::from([5]);
            let mut y = Modular::<Q>::from([9]);
            let x_plus_x = Modular::<Q>::from([10]);
            let x_plus_y = Modular::<Q>::from([1]);
            y += x;
            assert_eq!(x + x, x_plus_x);
            assert_eq!(y, x_plus_y);
        }
        #[test]
        fn test_add_zero() {
            const Q: u32 = 27;
            let x = Modular::<Q>::from([5]);
            let y = Modular::<Q>::from([0]);
            assert_eq!(x + y, x);
            assert_eq!(y + x, x);
        }
        #[test]
        fn test_sub_and_neg() {
            const Q: u32 = 31;
            let x = Modular::<Q>::from([5]);
            let y = Modular::<Q>::from([6]);
            let z = Modular::<Q>::from([1]);
            let x_minus_y = Modular::<Q>::from([Q - 1]);
            assert_eq!(x - y, x_minus_y);
            assert_eq!(x - y, -z);
        }
        #[test]
        fn test_mul() {
            const Q: u32 = 37;
            let x = Modular::<Q>::from([13]);
            let y = Modular::<Q>::from([5]);
            let z = Modular::<Q>::from([28]);
            assert_eq!(x * y, z);
        }
    }
}
