//! The `Zippable` trait for creating a `Zipper` from a tree-like structure.
//!
//! Provides arbitrary navigation of tree-like types. Allows for moving Up, Down, Left, Right, and Back based on arbitrary criteria.
//!
//! Ideal for algorithms that explore trees or applications where a user makes runtime navigation choices, such as:
//! - File and folder systems
//! - Directed graph node similarity
//!
//! Basic usage:
//!
//! ```
//! use zippered::zipper::{Step::*, *};
//!
//! #[derive(Clone)]
//! enum Tree {
//!     Node(usize),
//!     Branch(Vec<Tree>),
//! }
//!
//! impl Zippable for Tree {
//!     fn children(&self) -> Box<dyn Iterator<Item = Self> + '_> {
//!         match self {
//!             Tree::Node(_) => Box::new(std::iter::empty()),
//!             Tree::Branch(branch) => Box::new(branch.iter().cloned()),
//!         }
//!     }
//! }
//!
//! # fn main() -> Result<(), ZipperErr> {
//! # let tree = Tree::Branch(vec![Tree::Node(1), Tree::Node(2)]);
//!
//! // step around the tree
//! let value = tree.zipper().down()?.right()?.node;
//!
//! // or...
//! tree.zipper().travel(vec![Down, Right].into_iter());
//!
//! # Ok(())
//! # }
//! ```

use im::Vector;
use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::RefCell, fmt::Debug};

/// A trait for describing how a type can be made capable of be producing a [Zipper].
///
/// Any type that can describe iterating its children via an [Iterator] can be made [Zippable].
pub trait Zippable
where
    Self: Clone,
{
    /// Returns the children of the value. An empty [Iterator] can be used to signal that a node
    /// cannot or does not have children. See [std::iter::empty]
    fn children(&self) -> Box<dyn Iterator<Item = Self> + '_>;

    /// Creates and returns a [Zipper] for this value
    fn zipper(&self) -> Zipper<Self> {
        Zipper::new(self.clone())
    }
}

/// A unit of movement in a direction that a [Zipper] uses to traverse a [Zippable] tree.
///
/// See [Zipper::travel]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Step {
    Up,
    Down,
    Left,
    Right,
    Back,
}

type Path = Vector<Step>;

/// A record of the [Step]s taken by a [Zipper].
#[derive(Debug, Clone)]
struct History {
    path: Path,
    journey: Path,
}

impl History {
    fn new() -> Self {
        Self {
            path: Path::new(),
            journey: Path::new(),
        }
    }

    fn step(self, direction: Step) -> History {
        let mut next = self.clone();

        match direction {
            // A step in any of these directions effectively erases the end of the path as it is a backwards step
            Step::Back | Step::Left => {
                next.path.pop_back();
            }
            // we need to erase any previous Right steps
            Step::Up => {
                while next.path.last() == Some(&Step::Right) {
                    next.path.pop_back();
                }
                // finally, pop the Down step
                next.path.pop_back();
            }
            _ => {
                next.path.push_back(direction);
            }
        };

        next.journey.push_back(direction);

        next
    }
}

#[derive(Clone)]
struct SingletonNodeCache<T>
where
    T: Zippable,
{
    storage: Rc<RefCell<HashMap<Path, Zipper<T>>>>,
}

impl<T> SingletonNodeCache<T>
where
    T: Zippable,
{
    fn new() -> Self {
        Self {
            storage: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn find(&self, path: &Path) -> Option<Zipper<T>> {
        (*self.storage).borrow().get(path).cloned()
    }

    fn insert(&self, path: &Path, zipper: Zipper<T>) {
        self.storage
            .borrow_mut()
            .insert(path.clone(), zipper.clone());
    }
}

impl<T> std::fmt::Debug for SingletonNodeCache<T>
where
    T: Zippable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SingletonNodeCache")
            .field("entries", &(*self.storage).borrow().len())
            .finish()
    }
}

/// A cursor over a tree structure of [Zippable]s. Can be moved up, down, left, and right through
/// the tree and records traversal history as moves. A Zipper considers a tree's root to be at the top,
/// getting wider at the bottom. As such, Zipper cannot move `up`, `left`, or `right` from its starting position, nor
/// can it move `back` without moving elsewhere first.
///
/// Can [step](Zipper#method.step) or [travel](Zipper#method.travel) by processing [Step]s accordingly
#[derive(Clone)]
pub struct Zipper<T>
where
    T: Zippable,
{
    pub node: T,
    history: History,
    parent: Option<Rc<Zipper<T>>>,
    index_in_parent: Option<usize>,
    cache: SingletonNodeCache<T>,
}

impl<T> Zipper<T>
where
    T: Zippable,
{
    fn new(root: T) -> Self {
        Zipper {
            node: root,
            parent: None,
            index_in_parent: None,
            history: History::new(),
            cache: SingletonNodeCache::new(),
        }
    }

    /// Returns a new Zipper after moving down to this value's first child, or [ZipperErr.CannotMoveDown] if
    /// no children can or do exist.
    pub fn down(self) -> Result<Zipper<T>, ZipperErr> {
        // this is where we want to go
        let next_history = self.history.clone().step(Step::Down);
        // check cache and return if possible
        match self.cache.find(&next_history.path) {
            Some(mut cached) => {
                cached.history = next_history;
                return Ok(cached);
            }
            _ => (),
        }

        // see if we can move
        match self.node.children().next() {
            // we can
            Some(first) => {
                // see if we've been to this path before
                let next = Zipper {
                    node: first.clone(),
                    parent: Some(Rc::new(Zipper {
                        node: self.node.clone(),
                        parent: self.parent.clone(),
                        index_in_parent: self.index_in_parent,
                        history: self.history,
                        cache: self.cache.clone(),
                    })),
                    index_in_parent: Some(0),
                    history: next_history,
                    cache: self.cache.clone(),
                };

                // add to cache
                self.cache.insert(&next.history.path, next.clone());

                Ok(next)
            }
            None => Err(ZipperErr::CannotGoDown),
        }
    }

    /// Returns a new Zipper after moving up to this value's parent, or [ZipperErr.CannotMoveUp] if
    /// already at the root / top of the tree.
    pub fn up(self) -> Result<Zipper<T>, ZipperErr> {
        match self.parent {
            Some(ref parent) => Ok(Zipper {
                node: parent.node.clone(),
                parent: parent.parent.clone(),
                index_in_parent: parent.index_in_parent,
                history: self.history.step(Step::Up),
                cache: self.cache,
            }),
            None => Err(ZipperErr::CannotGoUp),
        }
    }

    /// Returns a new Zipper after moving right to this value's next sibling, or [ZipperErr.CannotMoveRight] if
    /// no right sibling exists.
    pub fn right(self) -> Result<Zipper<T>, ZipperErr> {
        // this is where we want to go
        let next_history = self.history.clone().step(Step::Right);
        // check cache and return if possible
        match self.cache.find(&next_history.path) {
            Some(mut cached) => {
                cached.history = next_history;
                return Ok(cached);
            }
            _ => (),
        }

        // see if we can move
        match (
            self.index_in_parent,
            self.parent.as_ref().map(|p| p.node.children()),
        ) {
            // we can
            (Some(index), Some(mut children)) => {
                let right_index = index + 1;
                match children.nth(right_index) {
                    Some(right) => {
                        let next = Zipper {
                            node: right,
                            parent: self.parent.clone(),
                            index_in_parent: right_index.into(),
                            history: next_history,
                            cache: self.cache.clone(),
                        };

                        // add to cache
                        self.cache.insert(&next.history.path, next.clone());

                        Ok(next)
                    }
                    _ => Err(ZipperErr::CannotGoRight),
                }
            }
            _ => Err(ZipperErr::CannotGoRight),
        }
    }

    /// Returns a new Zipper after moving left to this value's previous sibling, or [ZipperErr.CannotMoveLeft] if
    /// no left sibling exists.
    pub fn left(self) -> Result<Zipper<T>, ZipperErr> {
        // this is where we want to go
        let next_history = self.history.clone().step(Step::Left);
        // check cache and return if possible
        match self.cache.find(&next_history.path) {
            Some(mut cached) => {
                cached.history = next_history;
                return Ok(cached);
            }
            _ => (),
        }

        dbg!("We should really never be here if caching is working.");

        // see if we can move
        match (
            self.index_in_parent,
            self.parent.as_ref().map(|p| p.node.children()),
        ) {
            // we can
            (Some(index), Some(mut children)) if index > 0 => {
                let left_index = index - 1;
                match children.nth(left_index) {
                    Some(left) => {
                        let next = Zipper {
                            node: left,
                            parent: self.parent.clone(),
                            index_in_parent: Some(left_index),
                            history: next_history,
                            cache: self.cache.clone(),
                        };

                        // add to cache
                        self.cache.insert(&next.history.path, next.clone());

                        Ok(next)
                    }
                    None => Err(ZipperErr::CannotGoLeft),
                }
            }
            _ => Err(ZipperErr::CannotGoLeft),
        }
    }

    /// Returns a new Zipper after moving to the step prior the current value, or [ZipperErr.CannotMoveBack] if
    /// there have not yet been any [Step]s taken.
    pub fn back(self) -> Result<Zipper<T>, ZipperErr> {
        // this is where we want to go
        let next_history = self.history.clone().step(Step::Back);

        // check cache and return if possible
        match self.cache.find(&next_history.path) {
            Some(mut cached) => {
                cached.history = next_history;
                return Ok(cached);
            }
            _ => (),
        }

        // there is no traversal path, we are at the top, use parent if it exists
        match self.parent {
            Some(parent) if next_history.path.len() == 0 => {
                let mut next = parent.as_ref().clone();
                next.history = next_history;
                Ok(next)
            }
            _ => Err(ZipperErr::CannotGoBack),
        }
    }

    /// Return a new Zipper after taking a [Step] in the specified direction, or the relevant [ZipperErr] if
    /// no step can be taken.
    pub fn step(self, step: &Step) -> Result<Zipper<T>, ZipperErr> {
        match step {
            Step::Up => self.up(),
            Step::Down => self.down(),
            Step::Left => self.left(),
            Step::Right => self.right(),
            Step::Back => self.back(),
        }
    }

    /// Return a new Zipper after taking the specified sequence of [Step]s, or the relevant [ZipperErr] if
    /// any step cannot be taken.
    pub fn travel(self, path: impl Iterator<Item = Step>) -> Result<Zipper<T>, ZipperErr> {
        let mut zipper = self;

        for step in path {
            zipper = zipper.step(&step)?;
        }

        Ok(zipper)
    }

    /// The the most direct sequence of [Step]s taken to reach the [Zipper]'s current position.
    pub fn path(&self) -> impl Iterator<Item = Step> + '_ {
        self.history.path.iter().cloned()
    }

    /// The the exact sequence of [Step]s taken to reach the [Zipper]'s current position.
    ///
    /// This differs from [path](#method.path) in that it includes backtracking steps
    pub fn journey(&self) -> impl Iterator<Item = Step> + '_ {
        self.history.journey.iter().cloned()
    }

    /// Prints the current state of the [Zipper] via dbg
    pub fn show(self) -> Self
    where
        Self: Debug,
    {
        dbg!("{:#?}", &self);
        self
    }
}

/// Represents a [Zipper]'s inability to move in a given direction.
#[derive(Debug, Clone, Copy)]
pub enum ZipperErr {
    CannotGoUp,
    CannotGoLeft,
    CannotGoRight,
    CannotGoDown,
    CannotGoBack,
}
