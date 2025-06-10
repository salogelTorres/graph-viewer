use eframe::{App, run_native, NativeOptions};
use egui::{self, Pos2, Sense};
use petgraph::visit::EdgeRef;
use crate::graph::loader::{load_graphml, LoadError};
use crate::graph::state::ViewerState;

/// Application error
#[derive(Debug)]
pub enum ViewerError {
    Load(LoadError),
    Eframe(eframe::Error),
}
impl From<LoadError> for ViewerError { fn from(e: LoadError) -> Self { ViewerError::Load(e) } }
impl From<eframe::Error> for ViewerError { fn from(e: eframe::Error) -> Self { ViewerError::Eframe(e) } }

/// Runs the app
pub fn run_viewer(path: &str) -> Result<(), ViewerError> {
    let graph = load_graphml(path)?;
    

    let state = ViewerState::new(graph);
    let app = GraphViewerApp { state };
    let options = NativeOptions::default();
    run_native("Graph Viewer", options, Box::new(|_cc| Box::new(app)))?;
    Ok(())
}

struct GraphViewerApp { state: ViewerState }

impl App for GraphViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("GraphML Viewer"); ui.separator();
            let response = ui.allocate_rect(ui.available_rect_before_wrap(), Sense::drag());
            let painter = ui.painter_at(response.rect);

            // Initialize panning to center the graph
            if !self.state.initialized_view {
                self.state.pan[0] = response.rect.center().x;
                self.state.pan[1] = response.rect.center().y;
                self.state.initialized_view = true;
            }

            // Handle pan
            if response.dragged() {
                let delta = ui.input(|i| i.pointer.delta());
                self.state.pan[0] += delta.x;
                self.state.pan[1] += delta.y;
            }
            // Handle zoom
            let zoom_delta = ui.input(|i| i.scroll_delta.y);
            self.state.zoom *= 1.0 + zoom_delta * 0.01;
            // Draw edges
            for edge in self.state.graph.edge_references() {
                let [x1, y1] = self.state.graph[edge.source()].position;
                let [x2, y2] = self.state.graph[edge.target()].position;
                let p1 = Pos2::new(x1 * self.state.zoom + self.state.pan[0], y1 * self.state.zoom + self.state.pan[1]);
                let p2 = Pos2::new(x2 * self.state.zoom + self.state.pan[0], y2 * self.state.zoom + self.state.pan[1]);
                painter.line_segment([p1, p2], (2.0, ui.visuals().text_color()));
            }
            let node_radius = 25.0;
            // Draw nodes
            for node in self.state.graph.node_indices() {
                let [x, y] = self.state.graph[node].position;
                let pos = Pos2::new(x * self.state.zoom + self.state.pan[0], y * self.state.zoom + self.state.pan[1]);
                painter.circle_filled(pos, node_radius, ui.visuals().widgets.inactive.bg_fill);
                painter.text(pos, egui::Align2::CENTER_CENTER, &self.state.graph[node].label, egui::TextStyle::Body.resolve(ui.style()), ui.visuals().text_color());
            }
        });
        ctx.request_repaint();
    }
}
