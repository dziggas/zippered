use std::fmt::Debug;
use std::rc::Rc;

pub trait Zippable
where
    Self: Sized,
{
    fn children(&self) -> &[Self];

    fn zipper(&self) -> Zipper<Self> {
        Zipper::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct Zipper<'a, T>
where
    T: Zippable,
{
    node: &'a T,
    parent: Option<Rc<Zipper<'a, T>>>,
    index_in_parent: Option<usize>,
}

impl<'a, T> Zipper<'a, T>
where
    T: Zippable,
{
    fn new(root: &'a T) -> Self {
        Zipper {
            node: root,
            parent: None,
            index_in_parent: None,
        }
    }

    pub fn node(&self) -> &'a T {
        self.node
    }

    pub fn down(&self) -> Result<Zipper<'a, T>, &'static str> {
        match self.node.children().first() {
            Some(first) => Ok(Zipper {
                node: first,
                parent: Some(Rc::new(Zipper {
                    node: self.node,
                    parent: self.parent.clone(),
                    index_in_parent: self.index_in_parent,
                })),
                index_in_parent: Some(0),
            }),
            None => Err("cannot go down"),
        }
    }

    pub fn up(&self) -> Result<Zipper<'a, T>, &'static str> {
        match self.parent {
            Some(ref parent) => Ok(Zipper {
                node: parent.node,
                parent: parent.parent.clone(),
                index_in_parent: parent.index_in_parent,
            }),
            None => Err("cannot go up"),
        }
    }

    pub fn right(&self) -> Result<Zipper<'a, T>, &'static str> {
        match (&self.parent, self.index_in_parent) {
            (Some(parent), Some(index)) if index < parent.node.children().len() - 1 => {
                let right_index = index + 1;
                let right = 
                // safe because we already checked index being less than parent children length
                unsafe { parent.node.children().get_unchecked(index + 1) };

                Ok(Zipper {
                    node: right,
                    parent: self.parent.clone(),
                    index_in_parent: right_index.into(),
                })
            }
            _ => Err("cannot go right"),
        }
    }

    pub fn left(&self) -> Result<Zipper<'a, T>, &'static str> {
        match self.index_in_parent {
            Some(index) if index > 0 => {
                let left_index = index - 1;
                let left = 
                // safe because we already checked index being non-None and bounds being greater than zero
                unsafe {
                    self.parent
                        .as_ref()
                        .unwrap_unchecked()
                        .node
                        .children()
                        .get_unchecked(left_index)
                };

                Ok(Zipper {
                    node: left,
                    parent: self.parent.clone(),
                    index_in_parent: Some(left_index),
                })
            }
            _ => Err("cannot go left"),
        }
    }

    pub fn show(&self) -> &Self
    where
        Self: Debug,
    {
        println!("{:?}", self);
        self
    }
}

mod tests {
    use super::*;

    #[allow(dead_code)] // to ignore unused warnings of Tree enum
    #[derive(Debug, PartialEq, Eq)]
    enum Tree {
        Node(usize),
        Branch(Vec<Tree>),
    }

    impl Zippable for Tree {
        fn children(&self) -> &[Self] {
            match self {
                Tree::Node(_) => &[],
                Tree::Branch(c) => c.as_slice(),
            }
        }
    }

    #[test]
    fn down() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Node(1), Tree::Node(2)]);

        let result = tree.zipper().down()?.node();

        assert_eq!(*result, Tree::Node(1));
        Ok(())
    }

    #[test]
    fn down_down() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.down()?.node();

        assert_eq!(*result, Tree::Node(1));
        Ok(())
    }

    #[test]
    fn down_fail() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![]);

        let result = tree.zipper().down();

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn down_fail2() -> Result<(), &'static str> {
        let tree = Tree::Node(0);

        let result = tree.zipper().down();

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn down_up() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.up()?.node();

        assert_eq!(*result, tree);
        Ok(())
    }

    #[test]
    fn down_down_up() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.down()?.up()?.node();

        assert_eq!(*result, Tree::Branch(vec![Tree::Node(1)]));
        Ok(())
    }

    #[test]
    fn down_down_up_up() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.down()?.up()?.up()?.node();

        assert_eq!(*result, tree);
        Ok(())
    }

    #[test]
    fn down_right() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.right()?.node();

        assert_eq!(*result, Tree::Node(2));
        Ok(())
    }

    #[test]
    fn down_right_up() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.right()?.up()?.node();

        assert_eq!(*result, tree);
        Ok(())
    }

    #[test]
    fn down_right_fail() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)])]);

        let result = tree.zipper().down()?.right();

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn down_right_left() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.right()?.left()?.node();

        assert_eq!(*result, Tree::Branch(vec![Tree::Node(1)]));
        Ok(())
    }

    #[test]
    fn left_fail() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().left();

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn down_left_fail() -> Result<(), &'static str> {
        let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

        let result = tree.zipper().down()?.left();

        assert!(result.is_err());
        Ok(())
    }
}
