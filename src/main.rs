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
    let mut paths: Vec<Route> = Vec::new();
    Vec::new() 
  }

  fn outgoing(&self, from: Label) -> &Vec<Edge> {
    self.vertices.get(&from).unwrap()
  }
}

fn weight(route: Route) -> Weight {
  route.iter().map(|e| e.1).fold(0, |acc, item| acc + item)
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
  assert_eq!(vec![(6784u16, 2u16), (5069u16, 14u16), (4049u16, 14u16)], *graph.outgoing(3144))
}

