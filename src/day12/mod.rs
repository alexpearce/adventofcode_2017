//! Solutions for day 12.

use std::collections::HashSet;

struct Graph {
    nodes : Vec<Vec<usize>>
}

impl Graph {
    /// Return a new Graph object with capacity `size_hint`.
    ///
    /// The graph storage member will expand dynamically if more nodes are added than `size_hint`
    /// suggests.
    fn new(size_hint : usize) -> Graph {
        let nodes = Vec::with_capacity(size_hint);
        Graph { nodes }
    }

    /// Add a node to the graph, connected to `vertices`.
    fn add_node(&mut self, node : usize, vertices : Vec<usize>) {
        self.nodes.insert(node, vertices);
    }

    /// Return the list of nodes connected to `node`.
    ///
    /// Implements a non-recursive depth-first search algorithm.
    fn connected_nodes(&self, node : usize) -> Vec<usize> {
        // let mut ret = Vec::with_capacity(self.nodes.len());

        // Records whether a node has been visited
        let mut marked = vec![false; self.nodes.len()];
        // Stack of nodes that we're visiting
        let mut stack = Vec::new();

        marked[node] = true;
        stack.push(node);

        loop {
            // We pop and then insert the same element, so that we don't have to worry about the
            // borrow semantics of owning a reference to the last element (we just take ownership)
            let current_node = stack.pop().unwrap();
            for connected in self.nodes[current_node].iter() {
                if !marked[*connected] {
                    marked[*connected] = true;
                    stack.push(current_node);
                    stack.push(*connected);
                    break;
                }
            }
            if stack.is_empty() {
                break
            }
        }

        // Return the indicies of the marked list, which are the nodes that we visited
        marked.iter().enumerate()
              .filter_map(|(idx, x)| if *x { Some(idx) } else { None })
              .collect()
    }

    /// Return a vector of vectors of nodes representing all the disconnected subgraphs.
    fn disconnected_graphs(&self) -> Vec<Vec<usize>> {
        let mut graphs = vec![];

        let mut known_nodes : HashSet<usize> = HashSet::new();
        for (node, _) in self.nodes.iter().enumerate() {
            if !known_nodes.contains(&node) {
                let connected = self.connected_nodes(node);
                for n in connected.iter() {
                    known_nodes.insert(*n);
                }
                graphs.push(connected);
            }
        }

        graphs
    }
}

/// Return a `Graph` object made from the `nodes` string.
fn construct_graph(nodes : &str) -> Graph {
    let mut graph = Graph::new(nodes.lines().count());

    for line in nodes.lines() {
        let mut components = line.split(" <-> ");
        let node : usize = components.next().unwrap().parse().unwrap();
        let connected : Vec<_> = components.next().unwrap()
                                            .split(", ")
                                            .map(|x| x.parse().unwrap())
                                            .collect();
        graph.add_node(node, connected);
    }

    graph
}

/// Return a sorted list of nodes that are connected to node `n`.
///
/// # Examples
///
/// ```
/// use aoc17::day12::connected_nodes;
///
/// assert_eq!(connected_nodes("0 <-> 2
/// 1 <-> 1
/// 2 <-> 0, 3, 4
/// 3 <-> 2, 4
/// 4 <-> 2, 3, 6
/// 5 <-> 6
/// 6 <-> 4, 5", 0), vec![0, 2, 3, 4, 5, 6]);
/// ```
pub fn connected_nodes(nodes : &str, node : usize) -> Vec<usize> {
    let graph = construct_graph(nodes);
    graph.connected_nodes(node)
}

/// Return a list of lists of nodes that are subgraphs of `nodes`.
///
/// # Examples
///
/// ```
/// use aoc17::day12::disconnected_graphs;
///
/// assert_eq!(disconnected_graphs("0 <-> 2
/// 1 <-> 1
/// 2 <-> 0, 3, 4
/// 3 <-> 2, 4
/// 4 <-> 2, 3, 6
/// 5 <-> 6
/// 6 <-> 4, 5"), vec![vec![0, 2, 3, 4, 5, 6], vec![1]]);
/// ```
pub fn disconnected_graphs(nodes : &str) -> Vec<Vec<usize>> {
    let graph = construct_graph(nodes);
    graph.disconnected_graphs()
}
