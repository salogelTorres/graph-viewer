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
graph-viewer = { git = "https://github.com/salogelTorres/graph-viewer", tag = "v0.1.0" }# Or the latest version
```


### Programmatic Usage

To use the `graph-viewer` library within your own Rust application, you can load a GraphML file and run the viewer as follows:

First, ensure you have `graph-viewer` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
graph-viewer = { git = "https://github.com/salogelTorres/graph-viewer", tag = "v0.1.0" }# Or the latest version
```

Then, in your `main.rs` or `lib.rs` file:

```rust
use graph_viewer::graph::ui::run_viewer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph_file_path = "example.graphml";
    let _ = run_viewer(graph_file_path);
    Ok(())
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 
