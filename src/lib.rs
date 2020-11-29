#![feature(min_const_generics)]

use alga::general::*;
use num_traits::identities::{One, Zero};
use std::convert::From;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[macro_use]
extern crate alga_derive;

#[derive(Clone, Copy, PartialEq, Alga)]
#[alga_traits(RingCommutative(Additive, Multiplicative))]
struct Modular<const Q: u32>(u32);

impl<const Q: u32> From<[u32; 1]> for Modular<Q> {
    fn from(x: [u32; 1]) -> Self {
        Modular(x[0] % Q)
    }
}

impl<const Q: u32> Add<Modular<Q>> for Modular<Q> {
    type Output = Modular<Q>;
    fn add(self, other: Self) -> Self::Output {
        Modular::from([self.0 + other.0])
    }
}
impl<const Q: u32> AddAssign<Modular<Q>> for Modular<Q> {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other)
    }
}
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

impl<const Q: u32> SubAssign<Modular<Q>> for Modular<Q> {
    fn sub_assign(&mut self, other: Self) {
        *self = self.sub(other)
    }
}

impl<const Q: u32> Mul<Modular<Q>> for Modular<Q> {
    type Output = Modular<Q>;
    fn mul(self, other: Self) -> Self::Output {
        let x: u64 = self.0.into();
        let y: u64 = other.0.into();
        let modulus: u64 = Q.into();
        Modular::from([(x * y % modulus) as u32])
    }
}
impl<const Q: u32> MulAssign<Modular<Q>> for Modular<Q> {
    fn mul_assign(&mut self, other: Self) {
        *self = self.mul(other)
    }
}
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
