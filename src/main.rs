extern crate collections;
extern crate serialize;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::io::File;
use std::io::IoError;
use serialize::json;

type Label  = u16;
type Weight = u16;
type Edge   = (Label, Weight);
type Route  = Vec<Edge>;

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
    for edg in self.outgoing(from).iter() {
      frontier.push(vec![*edg]);
    }
    loop {
      let cheapest_route = match frontier.pop() {
        None => panic!("No route found!"),
        Some(p) => p
      };
      if (cheapest_route.end_label() == to) {
        let mut result: Vec<Label> = cheapest_route.iter().map(|edg| edg.0).collect();
        result.insert(0, from);
        return result;
      } else {
        let frontier_expansion = self.outgoing(cheapest_route.end_label());
        for &fe in frontier_expansion.iter() {
          let mut cheapest_route_clone = cheapest_route.clone();
          cheapest_route_clone.push(fe);
          frontier.push(cheapest_route_clone);
        }
      }
    }
  }

  fn outgoing(&self, from: Label) -> &Vec<Edge> {
    self.vertices.get(&from).unwrap()
  }
}

trait Weighted {
  fn weight(&self) -> Weight;
}

impl Weighted for Route {
  fn weight(&self) -> Weight {
    self.iter().map(|e| e.1).fold(0, |acc, item| acc + item)
  }
}

trait EndLabeled {
  fn end_label(&self) -> Label;
}

impl EndLabeled for Route {
  fn end_label(&self) -> Label {
    self.iter().last().unwrap().0
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
  assert_eq!(vec![(4384u16, 15u16)], *graph.outgoing(3138));
  assert_eq!(vec![(6784u16, 2u16), (5069u16, 14u16), (4049u16, 14u16)], *graph.outgoing(3144));
}

#[test]
fn test_dijkstra() {
  let graph = graph_from_json_file("graph.json");
  assert_eq!(vec![3144u16, 6784u16], graph.dijkstra(3144, 6784));
  assert_eq!(vec![3144u16, 6784u16], graph.dijkstra(201, 8775));
}
