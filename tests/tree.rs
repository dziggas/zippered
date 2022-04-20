use zippered::zipper::{Zippable, ZipperErr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Node(usize),
    Branch(Vec<Tree>),
}

impl Zippable for Tree {
    fn children(&self) -> Box<dyn Iterator<Item = Self> + '_> {
        match self {
            Tree::Node(_) => Box::new(std::iter::empty()),
            Tree::Branch(branch) => Box::new(branch.iter().cloned()),
        }
    }
}

#[test]
fn down() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Node(1), Tree::Node(2)]);

    let result = tree.zipper().down()?.node;

    assert_eq!(result, Tree::Node(1));
    Ok(())
}

#[test]
fn down_down() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.down()?.node;

    assert_eq!(result, Tree::Node(1));
    Ok(())
}

#[test]
fn down_fail() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![]);

    let result = tree.zipper().down();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_fail2() -> Result<(), ZipperErr> {
    let tree = Tree::Node(0);

    let result = tree.zipper().down();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_up() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.up()?.node;

    assert_eq!(result, tree);
    Ok(())
}

#[test]
fn down_back() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.back()?.node;

    assert_eq!(result, tree);
    Ok(())
}

#[test]
fn down_down_up() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.down()?.up()?.node;

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    Ok(())
}

#[test]
fn down_down_back() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.down()?.back()?.node;

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    Ok(())
}

#[test]
fn down_down_up_up() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.down()?.up()?.up()?.node;

    assert_eq!(result, tree);
    Ok(())
}

#[test]
fn down_right() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.right()?.node;

    assert_eq!(result, Tree::Node(2));
    Ok(())
}

#[test]
fn down_right_up() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.right()?.up()?.node;

    assert_eq!(result, tree);
    Ok(())
}

#[test]
fn down_right_fail() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)])]);

    let result = tree.zipper().down()?.right();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_right_left() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.right()?.left()?.node;

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    Ok(())
}

#[test]
fn down_right_back() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.right()?.back()?.node;

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    Ok(())
}

#[test]
fn left_fail() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().left();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn up_fail() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().up();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn back_fail() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().back();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_left_fail() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree.zipper().down()?.left();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_down_up_up_down_down() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let result = tree
        .zipper()
        .down()?
        .down()?
        .up()?
        .up()?
        .down()?
        .down()?
        .node;

    assert_eq!(result, Tree::Node(1));
    Ok(())
}
