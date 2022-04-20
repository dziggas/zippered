use zippered::zipper::Zippable;

#[derive(Debug, Clone)]
struct Usize(usize);

impl Zippable for Usize {
    fn children(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        Box::new(std::iter::empty())
    }
}

#[test]
fn test_all() {
    let u = Usize(42);

    assert!(u.zipper().up().is_err());
    assert!(u.zipper().down().is_err());
    assert!(u.zipper().left().is_err());
    assert!(u.zipper().right().is_err());
}
