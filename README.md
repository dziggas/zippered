# zippered
A simple, *read-only* implementation of [Huet's Zipper](https://www.st.cs.uni-saarland.de//edu/seminare/2005/advanced-fp/docs/huet-zipper.pdf)

## Zippers
[Zippers](https://en.wikipedia.org/wiki/Zipper_(data_structure)) allow for arbitrary navigation of a tree's nodes and branches. 

### What are Zippers good for?

A Zipper API roughly resembles:

`let value = tree.zipper().down()?.down()?.right()?.node;`

... which highlights that zippers are best for runtime navigation of a structure, where the shape of the data is not known at compile time. 

✅ File/folder navigation library  
✅ An algorithm that needs to "look around" in a tree of unknown shape  
❌ Pulling data out of JSON (the shape of JSON is usually a known contract that code can account for at compile time)

## Usage

To make something Zippable, implement the `Zippable` trait, which just requires specifying how children of that item can be iterated:

```
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
```

See the [tree tests](tests/tree.rs) for more details.

## Limitations

This implementation was born out of a specific read-only use case, and thus:

* There are no edit functions on the `Zipper`
* Currently, Zippable requires that `Self: Clone` because it was originally used with Rc/Arc
* `Zipper` does not memoize children traversal, which, if exploring in a tight radius of a node could cause re-iterations

It should be possible to remove any/all of these