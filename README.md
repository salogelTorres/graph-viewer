# Graph Viewer

Graph Viewer is a Rust library for visualizing GraphML files interactively. It allows you to load graph data and explore its structure with zooming and panning capabilities.

## Features

*   Loads GraphML files (nodes and edges).
*   Interactive graph visualization with `egui`.
*   Zooming and panning functionality.

## Installation

To use `graph-viewer` in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
graph-viewer = "0.1.0" # Replace with the actual version
```

## Usage

To run the graph viewer with an example GraphML file:

```bash
cargo run --bin visualize_graph -- <path_to_your_graphml_file>
```

Replace `<path_to_your_graphml_file>` with the actual path to your `.graphml` file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 