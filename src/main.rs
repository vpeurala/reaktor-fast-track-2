// TODO: Documentation of everything
extern crate collections;
extern crate serialize;

use graph::Edge;
use graph::Label;
use graph::Weight;
use graph::WeightedDirectedGraph;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::File;
use serialize::json;

mod graph;

#[deriving(Decodable, Encodable, Show)]
struct JsonEdge {
  from:   Label,
  to:     Label,
  weight: Weight 
}

#[deriving(Decodable, Encodable, Show)]
struct JsonJourney {
  from:  Label,
  to:    Label,
  route: Option<Vec<Label>>
}

#[deriving(Show)]
struct AdjacencyListBackedGraph {
  vertices: HashMap<Label, Vec<Edge>>
}

impl WeightedDirectedGraph for AdjacencyListBackedGraph {
  fn outgoing(&self, from: Label) -> Option<&Vec<Edge>> {
    self.vertices.get(&from)
  }
}

#[cfg(not(test))]
fn main() {
  let g: AdjacencyListBackedGraph = graph_from_json_file("graph.json");
  let journeys_in: Vec<JsonJourney> = journeys_from_json_file("journeys.json");
  let journeys_out: Vec<JsonJourney> = journeys_in.iter().map(|j| JsonJourney { from: j.from, to: j.to, route: Some(g.dijkstra(j.from, j.to).label_vec()) }).collect();
  println!("{}", json::encode(&journeys_out));
}

fn graph_from_json_file(file_name: &str) -> AdjacencyListBackedGraph {
  match File::open(&Path::new(file_name)).read_to_string() {
    Ok(s) => match json::decode(s.as_slice()) {
      Ok(v)  => make_graph(&v),
      Err(e) => panic!("Json decoder error, probably corrupt file: {}", e)
    },
    Err(e) => panic!("File {} could not be read: {}", file_name, e)
  }
}

fn journeys_from_json_file(file_name: &str) -> Vec<JsonJourney> {
  match File::open(&Path::new(file_name)).read_to_string() {
    Ok(s) => match json::decode(s.as_slice()) {
      Ok(v)  => v,
      Err(e) => panic!("Json decoder error, probably corrupt file: {}", e)
    },
    Err(e) => panic!("File {} could not be read: {}", file_name, e)
  }
}

fn make_graph(v: &Vec<JsonEdge>) -> AdjacencyListBackedGraph {
  let mut vertices: HashMap<Label, Vec<(Label, Weight)>> = HashMap::new();
  for &je in v.iter() {
    if !vertices.contains_key(&je.from) {
      vertices.insert(je.from, Vec::new());
    }
    vertices.get_mut(&je.from).unwrap().push((je.to, je.weight));
  }
  AdjacencyListBackedGraph { vertices: vertices }
}

#[test]
fn test_outgoing() {
  let graph = graph_from_json_file("graph.json");
  assert_eq!(vec![(4384, 15)], graph.outgoing_unvisited(3138, &HashSet::new()).unwrap());
  assert_eq!(vec![(6784, 2), (5069, 14), (4049, 14)], graph.outgoing_unvisited(3144, &HashSet::new()).unwrap());
}

#[test]
fn test_dijkstra() {
  let graph = graph_from_json_file("graph.json");
  assert_eq!(vec![3144, 6784], graph.dijkstra(3144, 6784).label_vec());
  assert_eq!(vec![201, 12, 38, 1410, 2982, 3926, 4702, 1336, 2019, 13894, 17745, 19375, 4821, 5265, 8775], graph.dijkstra(201, 8775).label_vec());
  assert_eq!(vec![23, 770, 1315, 2391, 3120, 3545, 8247, 8667, 23877], graph.dijkstra(23, 23877).label_vec());
  assert_eq!(vec![0, 6, 5, 16, 100, 777, 4410, 3287, 9102, 49486, 49900], graph.dijkstra(0, 49900).label_vec());
  assert_eq!(vec![7896, 21966, 20121, 3545, 422, 2, 48, 189, 297, 5547, 7542, 4361, 2417, 3681, 3693, 38949], graph.dijkstra(7896, 38949).label_vec());
}

