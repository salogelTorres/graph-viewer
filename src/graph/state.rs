use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

/// Node data
#[derive(Debug, Clone)]
pub struct NodeData { 
    pub id: String,
    pub label: String,
}

/// Edge data
#[derive(Debug, Clone)]
pub struct EdgeData { pub id: Option<String> }

/// Viewer state
pub struct ViewerState {
    pub graph: Graph<NodeData, EdgeData>,
    pub positions: HashMap<NodeIndex, [f32; 2]>,
    pub zoom: f32,
    pub pan: [f32; 2],
    pub initialized_view: bool,
}

impl ViewerState {
    pub fn new(graph: Graph<NodeData, EdgeData>) -> Self {
        let n = graph.node_count() as f32;
        let mut positions = HashMap::new();
        for (i, node) in graph.node_indices().enumerate() {
            let angle = i as f32 / n * std::f32::consts::TAU;
            positions.insert(node, [angle.cos() * 100.0, angle.sin() * 100.0]);
        }
        ViewerState { graph, positions, zoom: 2.0, pan: [0.0, 0.0], initialized_view: false }
    }
}