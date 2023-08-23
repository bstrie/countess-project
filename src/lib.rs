// TODO
// - operator overloads
//   - add
//   - sub
//   - mul
//   - div
//   - with self
// - deref impl
// - rest of the num interface
// - generic
// - error strategies
// - compaction
// - macro'd
// - intelligent checking strats

// TODO: comment about deliberately not overloading ops on underlying num types

use std::ops::{Add};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Foo {
    val: i8,
}

// TODO: naming comment
#[allow(non_snake_case)]
// TODO: inline comment
#[inline(always)]
pub fn Foo(val: i8) -> Foo {
    if val < Foo::MIN || val > Foo::MAX {
        panic!("Value out of bounds")
    } else {
        Foo { val }
    }
}

impl Foo {
    pub const MIN: i8 = -80;
    pub const MAX: i8 = 80;

    // TODO: inline comment
    #[inline(always)]
    // TODO: use macros to make the body identical to Foo() for function merging opts
    pub fn new(val: i8) -> Foo {
        Foo(val)
    }

    #[inline(always)]
    pub fn value(&self) -> i8 {
        self.val
    }
}

impl Add for Foo {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        let res = i8::checked_add(self.val, other.val).unwrap(); // panics on numeric overflow
        Foo(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let x = Foo(42);
        let y = Foo::new(42);
        assert_eq!(x.value(), y.value());
    }

    #[test]
    fn bounds() {
        assert_eq!(Foo::MIN, -80);
        assert_eq!(Foo::MAX, 80);
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
    fn add() {
        let x = Foo(-20);
        let y = Foo(30);
        let z = x + y;
        assert_eq!(10, z.value());
    }

    #[test]
    #[should_panic]
    fn overflow_add() {
        let _x = Foo(80) + Foo(80);
    }

    #[test]
    #[should_panic]
    fn underflow_add() {
        let _x = Foo(-80) + Foo(-80);
    }

    #[test]
    #[should_panic]
    fn min_oob_add() {
        let _x = Foo(-50) + Foo(-50);
    }

    #[test]
    #[should_panic]
    fn max_oob_add() {
        let _x = Foo(50) + Foo(50);
    }
}
