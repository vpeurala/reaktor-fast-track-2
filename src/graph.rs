// TODO: Documentation of everything
extern crate collections;
extern crate serialize;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

pub type Label  = uint;
pub type Weight = uint;
pub type Edge   = (Label, Weight);

#[deriving(Eq, PartialEq, Show)]
pub struct Route {
  start_label: Label,
  edges:       Vec<Edge>
}

pub trait WeightedDirectedGraph {
  fn dijkstra(&self, from: Label, to: Label) -> Route {
    let mut visited: HashSet<Label> = HashSet::new();
    let mut frontier: BinaryHeap<Route> = BinaryHeap::new();
    visited.insert(from);
    for edg in self.outgoing_unvisited(from, &visited).unwrap().iter() {
      frontier.push(Route { start_label: from, edges: vec![*edg] });
    }
    loop {
      let cheapest_route = match frontier.pop() {
        None => panic!("No route found!"),
        Some(p) => p
      };
      if cheapest_route.end_label() == to {
        return cheapest_route
      } else {
        visited.insert(cheapest_route.end_label());
        match self.outgoing_unvisited(cheapest_route.end_label(), &visited) {
          None => (),
          Some (frontier_expansion) => for &fe in frontier_expansion.iter() {
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

// TODO: impl Weighted for Edge
impl Weighted for Route {
  fn weight(&self) -> Weight {
    self.edges.iter().map(|e| e.1).fold(0, |acc, item| acc + item)
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
}

trait EndLabeled {
  fn end_label(&self) -> Label;
}

impl EndLabeled for Route {
  fn end_label(&self) -> Label {
    self.edges.iter().last().unwrap().0
  }
}

#[deriving(Show)]
pub struct AdjacencyListBackedGraph {
  // TODO: Should be private
  pub vertices: HashMap<Label, Vec<Edge>>
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

impl AdjacencyListBackedGraph {
  pub fn from_edges<T:EdgeSource>(v: Vec<T>) -> AdjacencyListBackedGraph {
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
