#![feature(min_const_generics)]

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::convert::From;

trait AdditiveAbelianGroup: Add + AddAssign + Sub + SubAssign + Neg + Sized {}
trait AbstractRing: AdditiveAbelianGroup + Mul + MulAssign {}
// Ring is modelled as "finite rank" of rank RANK, i.e. as a free module of rank RANK over (some
// algebraic subset of) `u32`
trait Ring<const N: usize>: AbstractRing + From<[u32; N]> {}

impl<R: AbstractRing + From<[u32; N]>, const RANK : usize> Ring<RANK> for R {}

mod modular_arithmetic {
    use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    #[derive(Debug, Copy, Clone, Default, Eq)]
    struct Modular<const Q: u32>(u32);
    // Can't generalize to arbitrary unsigned currently
    // as const generic parameter Q can't depend on other
    // generic types (such as T where T : Unsigned).

    impl<const Q: u32> From<[u32; 1]> for Modular<Q> {
        fn from(x: [u32;1]) -> Self {
            Modular(x[0] % Q)
        }
    }

    impl<const Q: u32> Add<Modular<Q>> for Modular<Q> {
        type Output = Modular<Q>;

        fn add(self, other: Self) -> Self::Output
        where
            Self: Sized,
        {
            Modular::from([self.0 + other.0])
        }
    }
    impl<const Q: u32> Mul<Modular<Q>> for Modular<Q> {
        type Output = Modular<Q>;

        fn mul(self, other: Self) -> Self::Output
        where
            Self: Sized,
        {
            // Multiplication can easily overflow datatype
            // Convert things up to u64's for multiplication, then back down after
            let x: u64 = self.0.into();
            let y: u64 = other.0.into();
            let modulus: u64 = Q.into();
            Modular::from([(x * y % modulus) as u32])
        }
    }
    impl<const Q: u32> Sub<Modular<Q>> for Modular<Q> {
        type Output = Modular<Q>;

        fn sub(self, other: Self) -> Self::Output
        where
            Self: Sized,
        {
            Modular::from([self.0 + other.neg().0])
        }
    }
    impl<const Q: u32> Neg for Modular<Q> {
        type Output = Modular<Q>;

        fn neg(self) -> Self::Output
        where
            Self: Sized,
        {
            Modular::from([Q - self.0])
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

    impl<const Q: u32> PartialEq for Modular<Q> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_add() {
            const Q: u32 = 13;
            let x = Modular::<Q>::from([5]);
            let y = Modular::<Q>::from([9]);
            let x_plus_x = Modular::<Q>::from([10]);
            let x_plus_y = Modular::<Q>::from([1]);
            assert_eq!(x + x, x_plus_x);
            assert_eq!(x + y, x_plus_y);
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
        #[test]
        fn test_add_assign() {
            const Q: u32 = 37;
            let x = Modular::<Q>::from([35]);
            let mut y = Modular::<Q>::from([2]);
            y += x;
            assert_eq!(y, Modular::<Q>::from([0]));
        }
    }
}

mod matrix {
    use super::Ring;
    use std::cell::Cell;
    /// An element of Mat_{N x M}(R)
    /// Each row is a (heap-allocated) array to make matrix-vector products Mx easier to compute
    struct Matrix<R: Ring<RANK>, const RANK: usize, const ROWS: usize, const COLS: usize>([Cell<[R ; COLS]>; ROWS]);
    impl<R: Ring<RANK>, const RANK: usize, const ROWS: usize, const COLS: usize> Matrix<R, RANK, ROWS, COLS> {
        fn new() {

        }

        fn zero() {
        }
    }

}

