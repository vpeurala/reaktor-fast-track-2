extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::borrow::Borrow;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;
use std::result::Result;

use serde::Deserialize;
use serde_json::de::from_str;
use serde_json::ser::to_string_pretty;

use graph::AdjacencyListBackedGraph;
use graph::EdgeSource;
use graph::Label;
use graph::Weight;
use graph::WeightedDirectedGraph;

pub mod graph;

#[derive(Serialize, Deserialize, Debug)]
struct JsonEdge {
    from: Label,
    to: Label,
    weight: Weight,
}

impl EdgeSource for JsonEdge {
    fn from(&self) -> Label { self.from }
    fn to(&self) -> Label { self.to }
    fn weight(&self) -> Weight { self.weight }
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonJourney {
    from: Label,
    to: Label,
    route: Option<Vec<Label>>,
}

#[cfg(not(test))]
fn main() {
    let g: AdjacencyListBackedGraph = graph_from_json_file("graph.json");
    let journeys_in: Vec<JsonJourney> = journeys_from_json_file("journeys.json");
    let journeys_out: Vec<JsonJourney> = journeys_in.iter().map(|j| JsonJourney { from: j.from, to: j.to, route: g.dijkstra(j.from, j.to).and_then(|g| Some(g.label_vec())) }).collect();
    //println!("{}", to_string_pretty(&journeys_out));
}

fn graph_from_json_file(file_name: &str) -> AdjacencyListBackedGraph {
    AdjacencyListBackedGraph::from_edges(decode_json_file::<JsonEdge>(file_name))
}

#[cfg(not(test))]
fn journeys_from_json_file(file_name: &str) -> Vec<JsonJourney> {
    decode_json_file(file_name)
}

fn decode_json_file<'a, T: Deserialize<'a>>(file_name: &str) -> Vec<T> {
    let json_string: &str = &read_file_to_string(file_name);
    // TODO Remove .
    //let json_string_ref: &'a str = &("[{\"from\": 1, \"to\": 2, \"weight\": 3}]");
    let json_string_ref: &'a str = &json_string.as_ref();
    from_str::<'a>(json_string_ref).unwrap()
}

fn read_file_to_string(file_name: &str) -> String {
    let file_open_result: Result<File, Error> = File::open(&Path::new(file_name));
    let file: File = file_open_result.unwrap();
    let mut buf_reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    let file_read_result: Result<usize, Error> = buf_reader.read_to_string(&mut contents);
    contents
}

#[test]
fn test_dijkstra() {
    let graph = graph_from_json_file("graph.json");
    assert_eq!(vec![3144, 6784], graph.dijkstra(3144, 6784).unwrap().label_vec());
    assert_eq!(vec![201, 12, 38, 1410, 2982, 3926, 4702, 1336, 2019, 13894, 17745, 19375, 4821, 5265, 8775], graph.dijkstra(201, 8775).unwrap().label_vec());
    assert_eq!(vec![23, 770, 1315, 2391, 3120, 3545, 8247, 8667, 23877], graph.dijkstra(23, 23877).unwrap().label_vec());
    assert_eq!(vec![0, 6, 5, 16, 100, 777, 4410, 3287, 9102, 49486, 49900], graph.dijkstra(0, 49900).unwrap().label_vec());
    assert_eq!(vec![7896, 21966, 20121, 3545, 422, 2, 48, 189, 297, 5547, 7542, 4361, 2417, 3681, 3693, 38949], graph.dijkstra(7896, 38949).unwrap().label_vec());
}
