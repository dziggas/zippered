use zippered::zipper::{Step::*, *};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Node(usize),
    Branch(Vec<Tree>),
}

impl Zippable for Tree {
    #[allow(refining_impl_trait)]
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

    let zipped = tree.zipper().down()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Node(1));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down]);
    assert_eq!(zipped.journey().collect::<Vec<Step>>(), vec![Down]);
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_down() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.down()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Node(1));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down, Down]);
    assert_eq!(zipped.journey().collect::<Vec<Step>>(), vec![Down, Down]);
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

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

    let zipped = tree.zipper().down()?.up()?;
    let result = zipped.node.clone();

    assert_eq!(result, tree);
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![]);
    assert_eq!(zipped.journey().collect::<Vec<Step>>(), vec![Down, Up]);
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_back() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.back()?;
    let result = zipped.node.clone();

    assert_eq!(result, tree);
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![]);
    assert_eq!(zipped.journey().collect::<Vec<Step>>(), vec![Down, Back]);
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_down_up() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.down()?.up()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down]);
    assert_eq!(
        zipped.journey().collect::<Vec<Step>>(),
        vec![Down, Down, Up]
    );
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_down_back() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.down()?.back()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down]);
    assert_eq!(
        zipped.journey().collect::<Vec<Step>>(),
        vec![Down, Down, Back]
    );
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_down_up_up() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.down()?.up()?.up()?;
    let result = zipped.node.clone();

    assert_eq!(result, tree);
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![]);
    assert_eq!(
        zipped.journey().collect::<Vec<Step>>(),
        vec![Down, Down, Up, Up]
    );
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_right() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.right()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Node(2));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down, Right]);
    assert_eq!(zipped.journey().collect::<Vec<Step>>(), vec![Down, Right]);
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_right_up() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.right()?.up()?;
    let result = zipped.node.clone();

    assert_eq!(result, tree);
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![]);
    assert_eq!(
        zipped.journey().collect::<Vec<Step>>(),
        vec![Down, Right, Up]
    );
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

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

    let zipped = tree.zipper().down()?.right()?.left()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down]);
    assert_eq!(
        zipped.journey().collect::<Vec<Step>>(),
        vec![Down, Right, Left]
    );
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}

#[test]
fn down_right_back() -> Result<(), ZipperErr> {
    let tree = Tree::Branch(vec![Tree::Branch(vec![Tree::Node(1)]), Tree::Node(2)]);

    let zipped = tree.zipper().down()?.right()?.back()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Branch(vec![Tree::Node(1)]));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down]);
    assert_eq!(
        zipped.journey().collect::<Vec<Step>>(),
        vec![Down, Right, Back]
    );
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

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

    let zipped = tree.zipper().down()?.down()?.up()?.up()?.down()?.down()?;
    let result = zipped.node.clone();

    assert_eq!(result, Tree::Node(1));
    assert_eq!(zipped.path().collect::<Vec<Step>>(), vec![Down, Down]);
    assert_eq!(
        zipped.journey().collect::<Vec<Step>>(),
        vec![Down, Down, Up, Up, Down, Down]
    );
    assert_eq!(result, tree.zipper().travel(zipped.path())?.node);
    assert_eq!(result, tree.zipper().travel(zipped.journey())?.node);

    Ok(())
}
