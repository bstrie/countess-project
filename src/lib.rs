// TODO
// - assoc consts
// - operator overloads
// - rest of the num interface
// - generic
// - macro'd
// - error strategies
// - compaction

pub struct Foo {
    val: u8,
}

// TODO: naming comment
// TODO: inline comment
#[allow(non_snake_case)]
#[inline(always)]
pub fn Foo(val: u8) -> Foo {
    Foo { val }
}

impl Foo {
    // TODO: inline comment
    #[inline(always)]
    pub fn new(val: u8) -> Foo {
        Foo { val }
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
    fn construct() {
        let x = Foo(42);
        let y = Foo::new(42);
        assert_eq!(x.value(), y.value());
    }
}
