extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod graph;

use graph::AdjacencyListBackedGraph;
use graph::EdgeSource;
use graph::Label;
use graph::Weight;
use graph::WeightedDirectedGraph;

use serde::Deserialize;
use serde_json::de::from_str;
#[cfg(not(test))]
use serde_json::ser::to_string_pretty;

#[cfg(not(test))]
use std::io::Stdout;
#[cfg(not(test))]
use std::io::StdoutLock;
#[cfg(not(test))]
use std::io::Write;
#[cfg(not(test))]
use std::io::stdout;

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
    let graph_json: &'static str = include_str!("graph.json");
    let journeys_json: &'static str = include_str!("journeys.json");
    let graph: AdjacencyListBackedGraph = graph_from_json(graph_json);
    let journeys_in: Vec<JsonJourney> = journeys_from_json(journeys_json);
    let journeys_out: Vec<JsonJourney> = journeys_in.
        iter().
        map(
            |j|
                {
                    JsonJourney
                        {
                            from: j.from,
                            to: j.to,
                            route: graph.dijkstra(j.from, j.to).
                                and_then(|g| { Some(g.label_vec()) }),
                        }
                }).
        collect();
    let journeys_out_pretty: String = to_string_pretty(&journeys_out).unwrap();
    let stdout: Stdout = stdout();
    let mut stdout_lock: StdoutLock = stdout.lock();
    let _ = stdout_lock.write_all(&journeys_out_pretty.into_bytes());
    let _ = stdout_lock.write(b"\n");
    let _ = stdout_lock.flush();
}

fn graph_from_json(json_str: &'static str) -> AdjacencyListBackedGraph {
    AdjacencyListBackedGraph::from_edges(decode_json::<JsonEdge>(json_str))
}

#[cfg(not(test))]
fn journeys_from_json(json_str: &'static str) -> Vec<JsonJourney> {
    decode_json(json_str)
}

fn decode_json<T: Deserialize<'static>>(json_str: &'static str) -> Vec<T> {
    from_str::<'static, Vec<T>>(json_str).unwrap()
}

#[test]
fn test_dijkstra() {
    let graph_json: &'static str = include_str!("graph.json");
    let graph: AdjacencyListBackedGraph = graph_from_json(graph_json);
    assert_eq!(vec![3144, 6784], graph.dijkstra(3144, 6784).unwrap().label_vec());
    assert_eq!(vec![201, 12, 38, 1410, 2982, 3926, 4702, 1336, 2019, 13894, 17745, 19375, 4821, 5265, 8775], graph.dijkstra(201, 8775).unwrap().label_vec());
    assert_eq!(vec![23, 770, 1315, 2391, 3120, 3545, 8247, 8667, 23877], graph.dijkstra(23, 23877).unwrap().label_vec());
    assert_eq!(vec![0, 6, 5, 16, 100, 777, 4410, 3287, 9102, 49486, 49900], graph.dijkstra(0, 49900).unwrap().label_vec());
    assert_eq!(vec![7896, 21966, 20121, 3545, 422, 2, 48, 189, 297, 5547, 7542, 4361, 2417, 3681, 3693, 38949], graph.dijkstra(7896, 38949).unwrap().label_vec());
}
