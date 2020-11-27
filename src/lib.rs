#![feature(min_const_generics)]

use std::ops::{Add, Mul, Sub};

trait Ring: Add + Sub + Mul + Sized {}

impl<R: Add + Sub + Mul + Sized> Ring for R {}

mod modular_arithmetic {
    use std::ops::{Add, Mul, Neg, Sub};

    #[derive(Debug, Copy, Clone, Default, Eq)]
    struct Modular<const Q: u32>(u32);
    // Can't generalize to arbitrary unsigned currently
    // as const generic parameter Q can't depend on other
    // generic types (such as T where T : Unsigned).

    impl<const Q: u32> Modular<Q> {
        fn new(x: u32) -> Self {
            Modular(x % Q)
        }
    }

    impl<const Q: u32> Add<Modular<Q>> for Modular<Q> {
        type Output = Modular<Q>;

        fn add(self, other: Self) -> Self
        where
            Self: Sized,
        {
            Modular::new(self.0 + other.0)
        }
    }

    impl<const Q: u32> Mul<Modular<Q>> for Modular<Q> {
        type Output = Modular<Q>;

        fn mul(self, other: Self) -> Self
        where
            Self: Sized,
        {
            // Multiplication can easily overflow datatype
            // Convert things up to u64's for multiplication, then back down after
            // Can change later to do strictly u32 multiplication by using Karabatsu
            let x : u64 = self.0.into();
            let y : u64 = other.0.into();
            let modulus : u64 = Q.into();
            Modular::new((x * y % modulus) as u32) 
        }
    }

    impl<const Q: u32> Sub<Modular<Q>> for Modular<Q> {
        type Output = Modular<Q>;

        fn sub(self, other: Self) -> Self
        where
            Self: Sized,
        {
            Modular::new(self.0 + Q - other.0)
        }
    }

    impl<const Q: u32> Neg for Modular<Q> {
        type Output = Modular<Q>;

        fn neg(self) -> Self
        where
            Self: Sized,
        {
            Modular::new(Q - self.0)
        }
    }

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
            let x = Modular::<Q>::new(5);
            let y = Modular::<Q>::new(9);
            let x_plus_x = Modular::<Q>::new(10);
            let x_plus_y = Modular::<Q>::new(1);
            assert_eq!(x + x, x_plus_x);
            assert_eq!(x + y, x_plus_y);
        }
        #[test]
        fn test_add_zero() {
            const Q: u32 = 27;
            let x = Modular::<Q>::new(5);
            let y = Modular::<Q>::new(0);
            assert_eq!(x + y, x);
            assert_eq!(y + x, x);
        }
        #[test]
        fn test_sub_and_neg() {
            const Q: u32 = 31;
            let x = Modular::<Q>::new(5);
            let y = Modular::<Q>::new(6);
            let z = Modular::<Q>::new(1);
            let x_minus_y = Modular::<Q>::new(Q - 1);
            assert_eq!(x - y, x_minus_y);
            assert_eq!(x - y, -z);
        }
        #[test]
        fn test_mul() {
            const Q: u32 = 37;
            let x = Modular::<Q>::new(13);
            let y = Modular::<Q>::new(5);
            let z = Modular::<Q>::new(28);
            assert_eq!(x * y, z);
        }
    }
}
