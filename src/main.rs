extern crate collections;
extern crate serialize;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::io::File;
use serialize::json;

type Label  = u16;
type Weight = u16;
type Edge   = (Label, Weight);

#[deriving(Eq, PartialEq)]
pub struct Route {
  start_label: Label,
  edges:       Vec<Edge>
}

#[deriving(Decodable, Encodable, Show)]
struct JsonEdge {
  from:   Label,
  to:     Label,
  weight: Weight 
}

#[deriving(Show)]
struct Graph {
  vertices: HashMap<Label, Vec<Edge>>
}

impl Graph {
  fn dijkstra(&self, from: Label, to: Label) -> Vec<Label> {
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
        let mut result: Vec<Label> = cheapest_route.edges.iter().map(|edg| edg.0).collect();
        result.insert(0, from);
        return result;
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
    println!("outgoing: {}", from);
    println!("visited: {}", visited.len());
    match self.vertices.get(&from) {
      None => None,
      Some(all_outgoing) => {
        let output: Vec<Edge> = all_outgoing.iter().filter(|e| !visited.contains(&e.0)).map(|e| e.clone()).collect();
        Some(output)
      }
    }
  }
}

trait Weighted {
  fn weight(&self) -> Weight;
}

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

trait EndLabeled {
  fn end_label(&self) -> Label;
}

impl EndLabeled for Route {
  fn end_label(&self) -> Label {
    self.edges.iter().last().unwrap().0
  }
}

#[cfg(not(test))]
fn main() {
  println!(graph_from_json_file("graph.json"));
}

fn graph_from_json_file(file_name: &str) -> Graph {
  match File::open(&Path::new(file_name)).read_to_string() {
    Ok(s) => match json::decode(s.as_slice()) {
      Ok(v)  => make_graph(&v),
      Err(e) => panic!("Json decoder error, probably corrupt file: {}", e)
    },
    Err(e) => panic!("File {} could not be read: {}", file_name, e)
  }
}

fn make_graph(v: &Vec<JsonEdge>) -> Graph {
  let mut vertices: HashMap<Label, Vec<(Label, Weight)>> = HashMap::new();
  for &je in v.iter() {
    if !vertices.contains_key(&je.from) {
      vertices.insert(je.from, Vec::new());
    }
    vertices.get_mut(&je.from).unwrap().push((je.to, je.weight));
  }
  Graph { vertices: vertices }
}

#[test]
fn test_outgoing() {
  let graph = graph_from_json_file("graph.json");
  assert_eq!(vec![(4384u16, 15u16)], graph.outgoing_unvisited(3138, &HashSet::new()).unwrap());
  assert_eq!(vec![(6784u16, 2u16), (5069u16, 14u16), (4049u16, 14u16)], graph.outgoing_unvisited(3144, &HashSet::new()).unwrap());
}

#[test]
fn test_dijkstra() {
  let graph = graph_from_json_file("graph.json");
  assert_eq!(vec![3144u16, 6784u16], graph.dijkstra(3144, 6784));
  assert_eq!(vec![3144u16, 6784u16], graph.dijkstra(201, 8775));
}
