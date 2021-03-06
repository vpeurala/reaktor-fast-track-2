use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

pub type Label = u32;
pub type Weight = u32;
pub type Edge = (Label, Weight);

#[derive(Eq, PartialEq, Debug)]
pub struct Route {
  start_label: Label,
  edges: Vec<Edge>,
}

pub trait WeightedDirectedGraph {
  fn dijkstra(&self, from: Label, to: Label) -> Option<Route> {
    let mut visited: HashSet<Label> = HashSet::new();
    let mut frontier: BinaryHeap<Route> = BinaryHeap::new();
    visited.insert(from);
    for edg in self.outgoing_unvisited(from, &visited).unwrap_or_default().iter() {
      frontier.push(Route { start_label: from, edges: vec![*edg] });
    }
    loop {
      let cheapest_route = match frontier.pop() {
        None => return None,
        Some(p) => p
      };
      if cheapest_route.end_label() == to {
        return Some(cheapest_route);
      } else {
        visited.insert(cheapest_route.end_label());
        match self.outgoing_unvisited(cheapest_route.end_label(), &visited) {
          None => (),
          Some(frontier_expansion) => for &fe in frontier_expansion.iter() {
            let mut cheapest_route_clone = cheapest_route.edges.clone();
            cheapest_route_clone.push(fe);
            frontier.push(Route { start_label: from, edges: cheapest_route_clone });
          }
        }
      }
    }
  }

  fn outgoing_unvisited(&self, from: Label, visited: &HashSet<Label>) -> Option<Vec<Edge>> {
    match self.outgoing(from) {
      None => None,
      Some(all_outgoing) => {
        let output: Vec<Edge> = all_outgoing.iter().filter(|e| !visited.contains(&e.0)).map(|e| e.clone()).collect();
        Some(output)
      }
    }
  }

  fn outgoing(&self, from: Label) -> Option<&Vec<Edge>>;
}

trait Weighted {
  fn weight(&self) -> Weight;
}

impl Weighted for Route {
  fn weight(&self) -> Weight {
    self.edges.iter().map(|e| e.weight()).fold(0, |sum, weight| sum + weight)
  }
}

impl Weighted for Edge {
  fn weight(&self) -> Weight {
    self.1
  }
}

impl Ord for Route {
  fn cmp(&self, other: &Route) -> Ordering {
    // Flipped around to get min-heap instead of max-heap.
    other.weight().cmp(&self.weight())
  }
}

impl PartialOrd for Route {
  fn partial_cmp(&self, other: &Route) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Route {
  pub fn label_vec(&self) -> Vec<Label> {
    let mut edge_labels: Vec<Label> = self.edges.iter().map(|e| e.0).collect();
    edge_labels.insert(0, self.start_label);
    edge_labels
  }

  fn end_label(&self) -> Label {
    self.edges.iter().last().unwrap_or(&(self.start_label, 0)).0
  }
}

#[derive(Debug)]
pub struct AdjacencyListBackedGraph {
  vertices: HashMap<Label, Vec<Edge>>
}

impl WeightedDirectedGraph for AdjacencyListBackedGraph {
  fn outgoing(&self, from: Label) -> Option<&Vec<Edge>> {
    self.vertices.get(&from)
  }
}

pub trait EdgeSource {
  fn from(&self) -> Label;
  fn to(&self) -> Label;
  fn weight(&self) -> Weight;
}

impl EdgeSource for (Label, Label, Weight) {
  fn from(&self) -> Label { self.0 }
  fn to(&self) -> Label { self.1 }
  fn weight(&self) -> Weight { self.2 }
}

impl AdjacencyListBackedGraph {
  pub fn from_edges<T: EdgeSource>(v: Vec<T>) -> AdjacencyListBackedGraph {
    let mut vertices: HashMap<Label, Vec<(Label, Weight)>> = HashMap::new();
    for je in v.iter() {
      if !vertices.contains_key(&je.from()) {
        vertices.insert(je.from(), Vec::new());
      }
      vertices.get_mut(&je.from()).unwrap().push((je.to(), je.weight()));
    }
    AdjacencyListBackedGraph { vertices: vertices }
  }
}

// -- Unit tests

#[cfg(test)]
///
///                             1
///                         +--------------------------------+
///                         v                                |
/// +---+  1   +---+  1   +---+  1   +---+  4   +---+  1   +---+
/// | 5 | ---> | 6 | ---> | 3 | ---> | 4 | <--- | 1 | ---> | 2 |
/// +---+      +---+      +---+      +---+      +---+      +---+
///                                               ^   1      |
///                                               +----------+
///
fn create_test_graph() -> AdjacencyListBackedGraph {
  AdjacencyListBackedGraph::from_edges(vec![
    (1, 2, 1),
    (2, 3, 1),
    (3, 4, 1),
    (2, 1, 1),
    (1, 4, 4),
    (5, 6, 1),
    (6, 3, 1)
  ])
}

#[test]
fn test_basic_cases() {
  let graph = create_test_graph();
  assert_eq!(vec![1, 2], graph.dijkstra(1, 2).unwrap().label_vec());
  assert_eq!(vec![1, 2, 3, 4], graph.dijkstra(1, 4).unwrap().label_vec());
  assert_eq!(None, graph.dijkstra(1, 5));
  assert_eq!(None, graph.dijkstra(8, 1));
  assert_eq!(None, graph.dijkstra(8, 9));
  assert_eq!(None, graph.dijkstra(1, 9));
}

