use graph_viewer::graph;

fn main() {
    let path = std::env::args().nth(1).expect("Ruta a GraphML");
    graph::run_viewer(&path).unwrap();
}
