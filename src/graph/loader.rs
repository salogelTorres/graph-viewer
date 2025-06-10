use std::{collections::HashMap, fs::File, io::BufReader};

use petgraph::graph::{Graph, NodeIndex};
use quick_xml::{events::Event, Reader};

use crate::graph::state::{EdgeData, NodeData};

/// Graph loading error
#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Xml(quick_xml::Error),
}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl From<quick_xml::Error> for LoadError {
    fn from(e: quick_xml::Error) -> Self {
        LoadError::Xml(e)
    }
}

/// Reads a simple GraphML file and constructs a `petgraph::Graph`.
/// Only considers:
///   * `<node id="...">` → creates node
///   * `<edge source="..." target="...">` → creates directed edge
pub fn load_graphml(path: &str) -> Result<Graph<NodeData, EdgeData>, LoadError> {
    // Graph to be returned
    let mut graph = Graph::<NodeData, EdgeData>::new();

    // Map id → NodeIndex to link edges
    let mut id_map: HashMap<String, NodeIndex> = HashMap::new();

    // State to know if we are inside the <graph> tag
    let mut in_graph = false;
    // State to capture the node label
    let mut current_node_id: Option<String> = None;
    let mut current_node_label: Option<String> = None;
    let mut expecting_label_text = false;

    // Configure the quick-xml reader
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.trim_text(true);

    let mut buf = Vec::<u8>::new();
    loop {
        let event = reader.read_event_into(&mut buf)?;
        match event {
            Event::Start(elem) => {
                let tag_name = String::from_utf8_lossy(elem.local_name().as_ref()).to_string();
                match tag_name.as_str() {
                    "graph" => {
                        in_graph = true;
                    }
                    "node" if in_graph => {
                        current_node_id = None;
                        current_node_label = None;
                        expecting_label_text = false;

                        if let Some(id_attr) = elem
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"id")
                        {
                            current_node_id = Some(String::from_utf8_lossy(&id_attr.value).to_string());
                        }
                    }
                    "data" if in_graph && current_node_id.is_some() => {
                        if let Some(key_attr) = elem
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"key")
                        {
                            if key_attr.value.as_ref() == b"label" {
                                expecting_label_text = true;
                            }
                        }
                    }
                    "edge" if in_graph => {
                        let mut source_id = None;
                        let mut target_id = None;

                        for attr in elem.attributes().filter_map(|a| a.ok()) {
                            match attr.key.as_ref() {
                                b"source" => {
                                    source_id =
                                        Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                b"target" => {
                                    target_id =
                                        Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                _ => {}
                            }
                        }

                        if let (Some(s), Some(t)) = (source_id, target_id) {
                            if let (Some(&s_idx), Some(&t_idx)) =
                                (id_map.get(&s), id_map.get(&t))
                            {
                                graph.add_edge(s_idx, t_idx, EdgeData { id: None });
                            }
                        }
                    }
                    _ => {} // Ignore any other start tag
                }
            },
            Event::Text(t) => {
                if expecting_label_text {
                    current_node_label = Some(String::from_utf8_lossy(t.as_ref()).to_string());
                    expecting_label_text = false; // We have already captured the label text
                }
            },
            Event::Empty(elem) => {
                let tag_name = String::from_utf8_lossy(elem.local_name().as_ref()).to_string();
                match tag_name.as_str() {
                    "node" if in_graph => {
                        let mut id = None;
                        let label = None;

                        for attr in elem.attributes().filter_map(|a| a.ok()) {
                            match attr.key.as_ref() {
                                b"id" => id = Some(String::from_utf8_lossy(&attr.value).to_string()),
                                _ => {}
                            }
                        }

                        if let Some(node_id) = id {
                            let idx = graph.add_node(NodeData { 
                                id: node_id.clone(), 
                                label: label.unwrap_or(node_id.clone()),
                                position: [0.0, 0.0],
                            });
                            id_map.insert(node_id, idx);
                        }
                    }
                    "edge" if in_graph => {
                        let mut source_id = None;
                        let mut target_id = None;

                        for attr in elem.attributes().filter_map(|a| a.ok()) {
                            match attr.key.as_ref() {
                                b"source" => {
                                    source_id =
                                        Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                b"target" => {
                                    target_id =
                                        Some(String::from_utf8_lossy(&attr.value).to_string())
                                }
                                _ => {}
                            }
                        }

                        if let (Some(s), Some(t)) = (source_id, target_id) {
                            if let (Some(&s_idx), Some(&t_idx)) =
                                (id_map.get(&s), id_map.get(&t))
                            {
                                graph.add_edge(s_idx, t_idx, EdgeData { id: None });
                            }
                        }
                    }
                    _ => {} // Ignore any other empty tag
                }
            },
            Event::End(elem) => {
                let tag_name = String::from_utf8_lossy(elem.local_name().as_ref()).to_string();
                if tag_name.as_str() == "graph" {
                    in_graph = false;
                } else if tag_name.as_str() == "node" && in_graph {
                    // End of a node tag (non-empty), finalize NodeData
                    if let Some(id) = current_node_id.take() {
                        let label = current_node_label.take().unwrap_or(id.clone());
                        let idx = graph.add_node(NodeData { id: id.clone(), label, position: [0.0, 0.0] });
                        id_map.insert(id, idx);
                    }
                }
            },
            // =================================================================
            // End of file
            Event::Eof => break,
            _ => {} // Ignore any other event
        }

        buf.clear(); // Important! Clear the buffer between reads
    }

    Ok(graph)
}
