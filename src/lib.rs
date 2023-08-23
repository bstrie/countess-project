// TODO
// - error strategies
// - macro'd
// - compaction
// - intelligent checking strats
// - configure whether or not to overload underlying numeric types
// - fuzzing

// TODO: comment about keeping the overflow detection algorithms deliberately simple

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
    pub fn new(v: i8) -> Foo {
        Foo(v)
    }

    #[inline(always)]
    pub fn val(&self) -> i8 {
        self.v
    }
}

// Add impls

impl std::ops::Add<Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn add(self, other: Foo) -> Self::Output {
        Foo(i8::checked_add(self.v, other.v).unwrap())
    }
}

impl std::ops::Add<&Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn add(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_add(self.v, other.v).unwrap())
    }
}

impl std::ops::Add<i8> for Foo {
    type Output = Foo;

    #[inline]
    fn add(self, other: i8) -> Self::Output {
        Foo(i8::checked_add(self.v, other).unwrap())
    }
}

impl std::ops::Add<&i8> for Foo {
    type Output = Foo;

    #[inline]
    fn add(self, other: &i8) -> Self::Output {
        Foo(i8::checked_add(self.v, *other).unwrap())
    }
}

impl std::ops::Add<Foo> for &Foo {
    type Output = Foo;

    #[inline(always)]
    fn add(self, other: Foo) -> Self::Output {
        Foo(i8::checked_add(self.v, other.v).unwrap())
    }
}

impl std::ops::Add<&Foo> for &Foo {
    type Output = Foo;

    #[inline]
    fn add(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_add(self.v, other.v).unwrap())
    }
}

impl std::ops::Add<i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn add(self, other: i8) -> Self::Output {
        Foo(i8::checked_add(self.v, other).unwrap())
    }
}

impl std::ops::Add<&i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn add(self, other: &i8) -> Self::Output {
        Foo(i8::checked_add(self.v, *other).unwrap())
    }
}

impl std::ops::Add<Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn add(self, other: Foo) -> Self::Output {
        Foo(i8::checked_add(self, other.v).unwrap())
    }
}

impl std::ops::Add<&Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn add(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_add(self, other.v).unwrap())
    }
}

impl std::ops::Add<Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn add(self, other: Foo) -> Self::Output {
        Foo(i8::checked_add(*self, other.v).unwrap())
    }
}

impl std::ops::Add<&Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn add(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_add(*self, other.v).unwrap())
    }
}

// Sub impls

impl std::ops::Sub<Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn sub(self, other: Foo) -> Self::Output {
        Foo(i8::checked_sub(self.v, other.v).unwrap())
    }
}

impl std::ops::Sub<&Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn sub(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_sub(self.v, other.v).unwrap())
    }
}

impl std::ops::Sub<i8> for Foo {
    type Output = Foo;

    #[inline]
    fn sub(self, other: i8) -> Self::Output {
        Foo(i8::checked_sub(self.v, other).unwrap())
    }
}

impl std::ops::Sub<&i8> for Foo {
    type Output = Foo;

    #[inline]
    fn sub(self, other: &i8) -> Self::Output {
        Foo(i8::checked_sub(self.v, *other).unwrap())
    }
}

impl std::ops::Sub<Foo> for &Foo {
    type Output = Foo;

    #[inline(always)]
    fn sub(self, other: Foo) -> Self::Output {
        Foo(i8::checked_sub(self.v, other.v).unwrap())
    }
}

impl std::ops::Sub<&Foo> for &Foo {
    type Output = Foo;

    #[inline]
    fn sub(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_sub(self.v, other.v).unwrap())
    }
}

impl std::ops::Sub<i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn sub(self, other: i8) -> Self::Output {
        Foo(i8::checked_sub(self.v, other).unwrap())
    }
}

impl std::ops::Sub<&i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn sub(self, other: &i8) -> Self::Output {
        Foo(i8::checked_sub(self.v, *other).unwrap())
    }
}

impl std::ops::Sub<Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn sub(self, other: Foo) -> Self::Output {
        Foo(i8::checked_sub(self, other.v).unwrap())
    }
}

impl std::ops::Sub<&Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn sub(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_sub(self, other.v).unwrap())
    }
}

impl std::ops::Sub<Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn sub(self, other: Foo) -> Self::Output {
        Foo(i8::checked_sub(*self, other.v).unwrap())
    }
}

impl std::ops::Sub<&Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn sub(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_sub(*self, other.v).unwrap())
    }
}

// Mul impls

impl std::ops::Mul<Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn mul(self, other: Foo) -> Self::Output {
        Foo(i8::checked_mul(self.v, other.v).unwrap())
    }
}

impl std::ops::Mul<&Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn mul(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_mul(self.v, other.v).unwrap())
    }
}

impl std::ops::Mul<i8> for Foo {
    type Output = Foo;

    #[inline]
    fn mul(self, other: i8) -> Self::Output {
        Foo(i8::checked_mul(self.v, other).unwrap())
    }
}

impl std::ops::Mul<&i8> for Foo {
    type Output = Foo;

    #[inline]
    fn mul(self, other: &i8) -> Self::Output {
        Foo(i8::checked_mul(self.v, *other).unwrap())
    }
}

impl std::ops::Mul<Foo> for &Foo {
    type Output = Foo;

    #[inline(always)]
    fn mul(self, other: Foo) -> Self::Output {
        Foo(i8::checked_mul(self.v, other.v).unwrap())
    }
}

impl std::ops::Mul<&Foo> for &Foo {
    type Output = Foo;

    #[inline]
    fn mul(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_mul(self.v, other.v).unwrap())
    }
}

impl std::ops::Mul<i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn mul(self, other: i8) -> Self::Output {
        Foo(i8::checked_mul(self.v, other).unwrap())
    }
}

impl std::ops::Mul<&i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn mul(self, other: &i8) -> Self::Output {
        Foo(i8::checked_mul(self.v, *other).unwrap())
    }
}

impl std::ops::Mul<Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn mul(self, other: Foo) -> Self::Output {
        Foo(i8::checked_mul(self, other.v).unwrap())
    }
}

impl std::ops::Mul<&Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn mul(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_mul(self, other.v).unwrap())
    }
}

impl std::ops::Mul<Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn mul(self, other: Foo) -> Self::Output {
        Foo(i8::checked_mul(*self, other.v).unwrap())
    }
}

impl std::ops::Mul<&Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn mul(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_mul(*self, other.v).unwrap())
    }
}

// Div impls

impl std::ops::Div<Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn div(self, other: Foo) -> Self::Output {
        Foo(i8::checked_div(self.v, other.v).unwrap())
    }
}

impl std::ops::Div<&Foo> for Foo {
    type Output = Foo;

    #[inline]
    fn div(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_div(self.v, other.v).unwrap())
    }
}

impl std::ops::Div<i8> for Foo {
    type Output = Foo;

    #[inline]
    fn div(self, other: i8) -> Self::Output {
        Foo(i8::checked_div(self.v, other).unwrap())
    }
}

impl std::ops::Div<&i8> for Foo {
    type Output = Foo;

    #[inline]
    fn div(self, other: &i8) -> Self::Output {
        Foo(i8::checked_div(self.v, *other).unwrap())
    }
}

impl std::ops::Div<Foo> for &Foo {
    type Output = Foo;

    #[inline(always)]
    fn div(self, other: Foo) -> Self::Output {
        Foo(i8::checked_div(self.v, other.v).unwrap())
    }
}

impl std::ops::Div<&Foo> for &Foo {
    type Output = Foo;

    #[inline]
    fn div(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_div(self.v, other.v).unwrap())
    }
}

impl std::ops::Div<i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn div(self, other: i8) -> Self::Output {
        Foo(i8::checked_div(self.v, other).unwrap())
    }
}

impl std::ops::Div<&i8> for &Foo {
    type Output = Foo;

    #[inline]
    fn div(self, other: &i8) -> Self::Output {
        Foo(i8::checked_div(self.v, *other).unwrap())
    }
}

impl std::ops::Div<Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn div(self, other: Foo) -> Self::Output {
        Foo(i8::checked_div(self, other.v).unwrap())
    }
}

impl std::ops::Div<&Foo> for i8 {
    type Output = Foo;

    #[inline]
    fn div(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_div(self, other.v).unwrap())
    }
}

impl std::ops::Div<Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn div(self, other: Foo) -> Self::Output {
        Foo(i8::checked_div(*self, other.v).unwrap())
    }
}

impl std::ops::Div<&Foo> for &i8 {
    type Output = Foo;

    #[inline]
    fn div(self, other: &Foo) -> Self::Output {
        Foo(i8::checked_div(*self, other.v).unwrap())
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
        fn impls() {
            assert_eq!(Foo(12)  + Foo(3),  Foo(15));
            assert_eq!(Foo(12)  + &Foo(3), Foo(15));
            assert_eq!(Foo(12)  + 3,       Foo(15));
            assert_eq!(Foo(12)  + &3,      Foo(15));
            assert_eq!(&Foo(12) + Foo(3),  Foo(15));
            assert_eq!(&Foo(12) + &Foo(3), Foo(15));
            assert_eq!(&Foo(12) + 3,       Foo(15));
            assert_eq!(&Foo(12) + &3,      Foo(15));
            assert_eq!(12       + Foo(3),  Foo(15));
            assert_eq!(12       + &Foo(3), Foo(15));
            assert_eq!(&12      + Foo(3),  Foo(15));
            assert_eq!(&12      + &Foo(3), Foo(15));
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
        fn impls() {
            assert_eq!(Foo(12)  - Foo(3),  Foo(9));
            assert_eq!(Foo(12)  - &Foo(3), Foo(9));
            assert_eq!(Foo(12)  - 3,       Foo(9));
            assert_eq!(Foo(12)  - &3,      Foo(9));
            assert_eq!(&Foo(12) - Foo(3),  Foo(9));
            assert_eq!(&Foo(12) - &Foo(3), Foo(9));
            assert_eq!(&Foo(12) - 3,       Foo(9));
            assert_eq!(&Foo(12) - &3,      Foo(9));
            assert_eq!(12       - Foo(3),  Foo(9));
            assert_eq!(12       - &Foo(3), Foo(9));
            assert_eq!(&12      - Foo(3),  Foo(9));
            assert_eq!(&12      - &Foo(3), Foo(9));
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
        fn impls() {
            assert_eq!(Foo(12)  * Foo(3),  Foo(36));
            assert_eq!(Foo(12)  * &Foo(3), Foo(36));
            assert_eq!(Foo(12)  * 3,       Foo(36));
            assert_eq!(Foo(12)  * &3,      Foo(36));
            assert_eq!(&Foo(12) * Foo(3),  Foo(36));
            assert_eq!(&Foo(12) * &Foo(3), Foo(36));
            assert_eq!(&Foo(12) * 3,       Foo(36));
            assert_eq!(&Foo(12) * &3,      Foo(36));
            assert_eq!(12       * Foo(3),  Foo(36));
            assert_eq!(12       * &Foo(3), Foo(36));
            assert_eq!(&12      * Foo(3),  Foo(36));
            assert_eq!(&12      * &Foo(3), Foo(36));
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
            let x = Foo(60);
            let y = Foo(-3);
            let z = x / y;
            assert_eq!(-20, z.val());
        }

        #[test]
        fn impls() {
            assert_eq!(Foo(12)  / Foo(3),  Foo(4));
            assert_eq!(Foo(12)  / &Foo(3), Foo(4));
            assert_eq!(Foo(12)  / 3,       Foo(4));
            assert_eq!(Foo(12)  / &3,      Foo(4));
            assert_eq!(&Foo(12) / Foo(3),  Foo(4));
            assert_eq!(&Foo(12) / &Foo(3), Foo(4));
            assert_eq!(&Foo(12) / 3,       Foo(4));
            assert_eq!(&Foo(12) / &3,      Foo(4));
            assert_eq!(12       / Foo(3),  Foo(4));
            assert_eq!(12       / &Foo(3), Foo(4));
            assert_eq!(&12      / Foo(3),  Foo(4));
            assert_eq!(&12      / &Foo(3), Foo(4));
        }

        // TODO: test to show that -128 / -1 panics on overflow
        // TODO: tests to show min/max oob panics
    }
}
