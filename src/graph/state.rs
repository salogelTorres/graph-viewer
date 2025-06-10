use petgraph::Graph;
use crate::graph::layout::apply_force_directed_layout;
use rand::Rng;
use rand::thread_rng;

/// Node data
#[derive(Debug, Clone)]
pub struct NodeData { 
    pub id: String,
    pub label: String,
    pub position: [f32; 2],
}

/// Edge data
#[derive(Debug, Clone)]
pub struct EdgeData { pub id: Option<String> }

/// Viewer state
pub struct ViewerState {
    pub graph: Graph<NodeData, EdgeData>,
    pub zoom: f32,
    pub pan: [f32; 2],
    pub initialized_view: bool,
}

impl ViewerState {
    pub fn new(mut graph: Graph<NodeData, EdgeData>) -> Self {
        let mut rng = thread_rng();
        for node_idx in graph.node_indices() {
            let position = [rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0)];
            graph[node_idx].position = position;
        }

        // Apply force-directed layout for better visualization
        apply_force_directed_layout(&mut graph, 200, 80.0, 10.0);

        // Calculate bounding box of the graph for initial zoom and pan
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for node_idx in graph.node_indices() {
            let pos = graph[node_idx].position;
            min_x = min_x.min(pos[0]);
            max_x = max_x.max(pos[0]);
            min_y = min_y.min(pos[1]);
            max_y = max_y.max(pos[1]);
        }

        let graph_width = max_x - min_x;
        let graph_height = max_y - min_y;

        // Target screen size (arbitrary, adjust as needed)
        let target_screen_width = 800.0;
        let target_screen_height = 600.0;

        let zoom_x = target_screen_width / graph_width;
        let zoom_y = target_screen_height / graph_height;

        let initial_zoom = zoom_x.min(zoom_y) * 0.8; // Factor de 0.8 para un pequeño margen

        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        // Calculate initial pan to center the graph
        let initial_pan_x = target_screen_width / 2.0 - center_x * initial_zoom;
        let initial_pan_y = target_screen_height / 2.0 - center_y * initial_zoom;

        ViewerState { graph, zoom: initial_zoom, pan: [initial_pan_x, initial_pan_y], initialized_view: true }
    }
}