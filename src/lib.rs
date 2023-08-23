// TODO
// - rest of the num interface
// - generic
// - error strategies
// - compaction
// - macro'd
// - intelligent checking strats

// TODO: comment about deliberately not overloading ops on underlying num types
// TODO: comment about keeping the overflow detection algorithms deliberately simple

use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Foo {
    v: i8,
}

// TODO: naming comment
#[allow(non_snake_case)]
// TODO: inline comment
#[inline(always)]
pub fn Foo(v: i8) -> Foo {
    if v < Foo::MIN || v > Foo::MAX {
        panic!("Value out of bounds")
    } else {
        Foo { v }
    }
}

impl Foo {
    pub const MIN: i8 = -80;
    pub const MAX: i8 = 80;

    // TODO: inline comment
    #[inline(always)]
    // TODO: use macros to make the body identical to Foo() for function merging opts
    pub fn new(v: i8) -> Foo {
        Foo(v)
    }

    #[inline(always)]
    pub fn val(&self) -> i8 {
        self.v
    }
}

impl Add for Foo {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        let res = i8::checked_add(self.v, other.v).unwrap(); // intentional panic
        Foo(res)
    }
}

impl Sub for Foo {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        let res = i8::checked_sub(self.v, other.v).unwrap(); // intentional panic
        Foo(res)
    }
}

impl Mul for Foo {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        let res = i8::checked_mul(self.v, other.v).unwrap(); // intentional panic
        Foo(res)
    }
}

impl Div for Foo {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        let res = i8::checked_div(self.v, other.v).unwrap(); // intentional panic
        Foo(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod basic {
        use super::*;

        #[test]
        fn bounds() {
            assert_eq!(Foo::MIN, -80);
            assert_eq!(Foo::MAX, 80);
            let _x = Foo(80);
            let _y = Foo(-80);
        }

        #[test]
        fn construction() {
            let x = Foo(42);
            let y = Foo::new(42);
            assert_eq!(x.val(), y.val());
        }

        #[test]
        #[should_panic]
        fn min_oob_construction() {
            let _x = Foo(-100);
        }

        #[test]
        #[should_panic]
        fn max_oob_construction() {
            let _x = Foo(100);
        }

        #[test]
        fn derives() {
            let x = Foo(42);
            let y = x; let z = x; // Copy
            let a = x.clone(); // Clone
            assert_eq!(x, y); // PartialEq
            assert!(z < Foo(43)); // PartialOrd
            println!("{a:?}"); // Debug
        }
    }

    mod add {
        use super::*;

        #[test]
        fn success() {
            let x = Foo(-20);
            let y = Foo(30);
            let z = x + y;
            assert_eq!(10, z.val());
        }

        #[test]
        #[should_panic]
        fn underflow() {
            let _x = Foo(-80) + Foo(-80);
        }

        #[test]
        #[should_panic]
        fn overflow() {
            let _x = Foo(80) + Foo(80);
        }

        #[test]
        #[should_panic]
        fn min_oob() {
            let _x = Foo(-50) + Foo(-50);
        }

        #[test]
        #[should_panic]
        fn max_oob() {
            let _x = Foo(50) + Foo(50);
        }
    }

    mod sub {
        use super::*;

        #[test]
        fn success() {
            let x = Foo(-20);
            let y = Foo(30);
            let z = x - y;
            assert_eq!(-50, z.val());
        }

        #[test]
        #[should_panic]
        fn underflow() {
            let _x = Foo(-80) - Foo(80);
        }

        #[test]
        #[should_panic]
        fn overflow() {
            let _x = Foo(80) - Foo(-80);
        }

        #[test]
        #[should_panic]
        fn min_oob() {
            let _x = Foo(-50) - Foo(50);
        }

        #[test]
        #[should_panic]
        fn max_oob() {
            let _x = Foo(50) - Foo(-50);
        }
    }

    mod mul {
        use super::*;

        #[test]
        fn success() {
            let x = Foo(2);
            let y = Foo(30);
            let z = x * y;
            assert_eq!(60, z.val());
        }

        #[test]
        #[should_panic]
        fn underflow() {
            let _x = Foo(20) * Foo(-20);
        }

        #[test]
        #[should_panic]
        fn overflow() {
            let _x = Foo(20) * Foo(20);
        }

        #[test]
        #[should_panic]
        fn min_oob() {
            let _x = Foo(10) * Foo(-10);
        }

        #[test]
        #[should_panic]
        fn max_oob() {
            let _x = Foo(10) * Foo(10);
        }
    }

    mod div {
        use super::*;

        #[test]
        fn success() {
            let x = Foo(-20);
            let y = Foo(30);
            let z = x + y;
            assert_eq!(10, z.val());
        }

        // TODO: test to show that -128 / -1 panics on overflow
        // TODO: tests to show min/max oob panics
    }
}
