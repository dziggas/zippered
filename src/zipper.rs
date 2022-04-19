use im::Vector;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Zippable
where
    Self: Clone,
{
    fn children(&self) -> Box<dyn Iterator<Item = Self> + '_>;

    fn zipper(&self) -> Zipper<Self> {
        Zipper::new(self.clone())
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Step {
    Up,
    Down,
    Left,
    Right,
    Back,
}

#[derive(Debug, Clone)]
pub struct History {
    path: Vector<Step>,
    journey: Vector<Step>,
}

type Path = Vector<Step>;

impl History {
    fn new() -> Self {
        Self {
            path: Path::new(),
            journey: Vector::new(),
        }
    }

    fn step(self, direction: Step) -> History {
        let mut next = self.clone();

        match direction {
            Step::Back => {
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

#[derive(Debug, Clone)]
pub struct Zipper<T>
where
    T: Zippable,
{
    pub node: T,
    pub history: History,
    parent: Option<Rc<Zipper<T>>>,
    index_in_parent: Option<usize>,
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
        }
    }

    pub fn down(self) -> Result<Zipper<T>, ZipperErr> {
        match self.node.children().next() {
            Some(first) => {
                let next = Zipper {
                    node: first.clone(),
                    parent: Some(Rc::new(Zipper {
                        node: self.node.clone(),
                        parent: self.parent.clone(),
                        index_in_parent: self.index_in_parent,
                        history: self.history.clone(),
                    })),
                    index_in_parent: Some(0),
                    history: self.history.step(Step::Down),
                };

                Ok(next)
            }
            None => Err(ZipperErr::CannotGoDown),
        }
    }

    pub fn up(self) -> Result<Zipper<T>, ZipperErr> {
        match self.parent {
            Some(ref parent) => Ok(Zipper {
                node: parent.node.clone(),
                parent: parent.parent.clone(),
                index_in_parent: parent.index_in_parent,
                history: self.history.step(Step::Up),
            }),
            None => Err(ZipperErr::CannotGoUp),
        }
    }

    pub fn right(self) -> Result<Zipper<T>, ZipperErr> {
        match (
            self.index_in_parent,
            self.parent.as_ref().map(|p| p.node.children()),
        ) {
            (Some(index), Some(mut children)) => {
                let right_index = index + 1;
                match children.nth(right_index) {
                    Some(right) => {
                        let next = Zipper {
                            node: right,
                            parent: self.parent.clone(),
                            index_in_parent: right_index.into(),
                            history: self.history.step(Step::Right),
                        };

                        Ok(next)
                    }
                    _ => Err(ZipperErr::CannotGoRight),
                }
            }
            _ => Err(ZipperErr::CannotGoRight),
        }
    }

    pub fn left(self) -> Result<Zipper<T>, ZipperErr> {
        match (
            self.index_in_parent,
            self.parent.as_ref().map(|p| p.node.children()),
        ) {
            (Some(index), Some(mut children)) if index > 0 => {
                let left_index = index - 1;
                match children.nth(left_index) {
                    Some(left) => {
                        let next = Zipper {
                            node: left,
                            parent: self.parent.clone(),
                            index_in_parent: Some(left_index),
                            history: self.history.step(Step::Left),
                        };
                        Ok(next)
                    }
                    None => Err(ZipperErr::CannotGoLeft),
                }
            }
            _ => Err(ZipperErr::CannotGoLeft),
        }
    }

    pub fn show(self) -> Self
    where
        Self: Debug,
    {
        println!("{:#?}", &self);
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ZipperErr {
    CannotGoUp,
    CannotGoLeft,
    CannotGoRight,
    CannotGoDown,
}
