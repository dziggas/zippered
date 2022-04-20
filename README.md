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

## Features & Limitations

This implementation was born out of a specific, read-only use case, and thus:

* There are no edit functions on the `Zipper`
* There is bookkeeping within Zipper to memoize historic traversal. Zippable only requires that nodes provide an Iterator over that node's children. This allows a variety of lazy, flexible `Zippable::children` implementations, but precludes Zipper from internally using something like `parent.children[current_position - 1]` to efficiently move left. Memoization solves this issue at the cost of some space
  * The bookkeeping allows for implementing `back`, which is atypical for Zippers
  * It also allows for storage and retreival of `Path` and `Journey` types where `Path` is a direct navigation path to a node in the `Zipper` and where `Journey` is the entire traversal/movement history
* Currently, Zippable requires `Self: Clone` because it was originally used with Rc/Arc
  * It is highly recommended that Zippable impl targets are trivially `Clone` or wrapped in Rc/Arc