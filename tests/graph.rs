use petgraph::{graph::NodeIndex, EdgeDirection::Outgoing, Graph};
use zippered::zipper::{Zippable, ZipperErr};

#[derive(Debug, Clone)]
struct ZippableGraph<'g> {
    graph: &'g Graph<usize, usize, petgraph::Directed>,
    node_idx: NodeIndex,
}

impl<'g> ZippableGraph<'g> {
    fn new(graph: &'g Graph<usize, usize, petgraph::Directed>, node_idx: NodeIndex) -> Self {
        Self { graph, node_idx }
    }

    fn value(&self) -> usize {
        self.graph.node_weight(self.node_idx).cloned().unwrap_or(0)
    }
}

impl<'g> Zippable for ZippableGraph<'g> {
    fn children(&self) -> impl Iterator<Item = Self> + '_ {
        Box::new(
            self.graph
                .neighbors_directed(self.node_idx, Outgoing)
                .map(|node| ZippableGraph::new(self.graph, node))
                // should not be necessary to do this normally, but neighbors are iterated in reverse-add order
                // in petgraph, so we collect and reverse again here to make the tests easier to follow
                .collect::<Vec<_>>()
                .into_iter()
                .rev(),
        )
    }
}

#[test]
fn down() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, one), (root, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.node;

    assert_eq!(result.value(), 1);
    Ok(())
}

#[test]
fn down_down() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let parent = graph.add_node(42);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, parent), (parent, one), (parent, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.down()?.node;

    assert_eq!(result.value(), 1);
    Ok(())
}

#[test]
fn down_fail() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_up() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let parent = graph.add_node(42);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, parent), (parent, one), (parent, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.up()?.node;

    assert_eq!(result.value(), 0);
    Ok(())
}

#[test]
fn down_down_up() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let parent = graph.add_node(42);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, parent), (parent, one), (parent, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.down()?.up()?.node;

    assert_eq!(result.value(), 42);
    Ok(())
}

#[test]
fn down_down_up_up() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let parent = graph.add_node(42);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, parent), (parent, one), (parent, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.down()?.up()?.up()?.node;

    assert_eq!(result.value(), 0);
    Ok(())
}

#[test]
fn down_right() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, one), (root, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.right()?.node;

    assert_eq!(result.value(), 2);
    Ok(())
}

#[test]
fn down_right_up() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, one), (root, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.right()?.up()?.node;

    assert_eq!(result.value(), 0);
    Ok(())
}

#[test]
fn down_right_fail() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let one = graph.add_node(1);
    graph.extend_with_edges([(root, one)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.right();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_right_left() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, one), (root, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.right()?.left()?.node;

    assert_eq!(result.value(), 1);
    Ok(())
}

#[test]
fn left_fail() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let one = graph.add_node(1);
    let two = graph.add_node(2);
    graph.extend_with_edges([(root, one), (root, two)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().left();

    assert!(result.is_err());
    Ok(())
}

#[test]
fn down_left_fail() -> Result<(), ZipperErr> {
    let mut graph = Graph::<usize, usize, petgraph::Directed>::new();
    let root = graph.add_node(0);
    let one = graph.add_node(1);
    graph.extend_with_edges([(root, one)]);

    let zippable = ZippableGraph::new(&graph, root);

    let result = zippable.zipper().down()?.left();

    assert!(result.is_err());
    Ok(())
}
