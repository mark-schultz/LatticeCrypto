//! Matrix-Matrix and Matrix-Vector products, where matrices are defined over finite-rank
//! commutative rings

use crate::rings::FinRankCRing;
use alga::general::*;
use num_traits::identities::{One, Zero};
use std::convert::From;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, PartialEq, Debug, Alga)]
#[alga_traits(GroupAbelian(Additive))]
struct Vector<R: Sized, const DIM: usize>(
    [R; DIM],
);

impl<R : FinRankCRing<N>, const N : u32, const DIM : usize> Add<Vector<R, DIM>> for Vector<R, DIM> {
    type Output = Vector<R, DIM>;
    fn add(self, other : Self) -> Self::Output {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a+b).collect()
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
        self + other.neg()
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
// TODO: Write macros to generate the below
// from Add, AddAssign, Mul, MulAssign, Sub, One, and Zero
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
