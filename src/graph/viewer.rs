use petgraph::Graph;
use petgraph_graphml::GraphML;
use quick_xml::Reader;
use quick_xml::events::Event;
use eframe::{egui, epi};
use std::io::BufReader;
use std::fs::File;

/// Node data for the graph (can be extended with more fields)
#[derive(Debug)]
pub struct NodeData {
    pub id: String,
}

/// Edge data for the graph (can be extended with more fields)
#[derive(Debug)]
pub struct EdgeData {
    pub id: String,
}

/// Error loading GraphML
#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Xml(quick_xml::Error),
}

impl From<std::io::Error> for LoadError {
    fn from(err: std::io::Error) -> Self {
        LoadError::Io(err)
    }
}

impl From<quick_xml::Error> for LoadError {
    fn from(err: quick_xml::Error) -> Self {
        LoadError::Xml(err)
    }
}

/// Viewer error
#[derive(Debug)]
pub enum ViewerError {
    Load(LoadError),
    Eframe(eframe::Error),
}

impl From<LoadError> for ViewerError {
    fn from(err: LoadError) -> Self {
        ViewerError::Load(err)
    }
}

impl From<eframe::Error> for ViewerError {
    fn from(err: eframe::Error) -> Self {
        ViewerError::Eframe(err)
    }
}

/// Loads a GraphML file and constructs a graph
pub fn load_graphml(path: &str) -> Result<Graph<NodeData, EdgeData>, LoadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // TODO: implement parsing with quick_xml and petgraph-graphml
    // This is just a stub
    let mut graph = Graph::<NodeData, EdgeData>::new();
    Ok(graph)
}

/// Viewer application state
pub struct ViewerState {
    pub graph: Graph<NodeData, EdgeData>,
    pub selected_node: Option<petgraph::graph::NodeIndex>,
    pub zoom: f32,
    pub pan: (f32, f32),
}

impl ViewerState {
    pub fn new(graph: Graph<NodeData, EdgeData>) -> Self {
        Self {
            graph,
            selected_node: None,
            zoom: 1.0,
            pan: (0.0, 0.0),
        }
    }
}

/// Main function to start the application
pub fn run_viewer(path: &str) -> Result<(), ViewerError> {
    let graph = load_graphml(path)?;
    dbg!(graph.node_count(), graph.edge_count());
    let state = ViewerState::new(graph);
    let app = GraphViewerApp { state };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Graph Viewer", native_options, Box::new(|_cc| Box::new(app)))?;
    Ok(())
}

/// Egui application structure
struct GraphViewerApp {
    state: ViewerState,
}

impl epi::App for GraphViewerApp {
    fn name(&self) -> &str {
        "Graph Viewer"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GraphML Viewer");
            ui.separator();
            // TODO: draw nodes and edges using egui UI
            // fn ui(&mut self, ui: &mut egui::Ui) { ... }
        });
    }
}
