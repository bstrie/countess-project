// TODO
// - operator overloads
//   - add
//   - sub
//   - mul
//   - div
//   - with self
//   - with underlying type
// - deref impl
// - rest of the num interface
// - generic
// - error strategies
// - compaction
// - macro'd
// - intelligent checking strats

pub struct Foo {
    val: u8,
}

// TODO: naming comment
#[allow(non_snake_case)]
// TODO: inline comment
#[inline(always)]
pub fn Foo(val: u8) -> Foo {
    if val < Foo::MIN || val > Foo::MAX {
        panic!("Value out of bounds")
    } else {
        Foo { val }
    }
}

impl Foo {
    pub const MIN: u8 = 20;
    pub const MAX: u8 = 80;

    // TODO: inline comment
    #[inline(always)]
    // TODO: use macros to make the body identical to Foo() for function merging opts
    pub fn new(val: u8) -> Foo {
        Foo(val)
    }

    #[inline(always)]
    pub fn value(&self) -> u8 {
        self.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let x = Foo(42);
        let y = Foo::new(42);
        assert_eq!(x.value(), y.value());
    }

    #[test]
    fn bounds() {
        assert_eq!(Foo::MIN, 20);
        assert_eq!(Foo::MAX, 80);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_construction() {
        let _x = Foo(200);
    }
}
