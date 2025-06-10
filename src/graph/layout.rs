use petgraph::graph::{Graph, NodeIndex};
use crate::graph::state::{NodeData, EdgeData};
use std::collections::HashMap;

pub fn apply_force_directed_layout(
    graph: &mut Graph<NodeData, EdgeData>,
    iterations: usize,
    k: f32, // Optimal distance
    c: f32, // Learning rate / cooling factor
) {
    let mut displacements: HashMap<NodeIndex, [f32; 2]> = HashMap::new();

    for _i in 0..iterations {
        // Initialize displacements for all nodes
        for node_idx in graph.node_indices() {
            displacements.insert(node_idx, [0.0, 0.0]);
        }

        // Calculate repulsive forces between all pairs of nodes
        for i_idx in graph.node_indices() {
            for j_idx in graph.node_indices() {
                if i_idx == j_idx { continue; }

                let p_i = graph[i_idx].position;
                let p_j = graph[j_idx].position;

                let delta = [p_j[0] - p_i[0], p_j[1] - p_i[1]];
                let distance = (delta[0].powi(2) + delta[1].powi(2)).sqrt();

                if distance > 0.0 {
                    let force = k.powi(2) / distance;
                    let displacement = [
                        delta[0] / distance * force,
                        delta[1] / distance * force,
                    ];
                    displacements.get_mut(&i_idx).unwrap()[0] -= displacement[0];
                    displacements.get_mut(&i_idx).unwrap()[1] -= displacement[1];
                    displacements.get_mut(&j_idx).unwrap()[0] += displacement[0];
                    displacements.get_mut(&j_idx).unwrap()[1] += displacement[1];
                }
            }
        }

        // Calculate attractive forces between connected nodes
        for edge_idx in graph.edge_indices() {
            let (source_idx, target_idx) = graph.edge_endpoints(edge_idx).unwrap();
            let p_source = graph[source_idx].position;
            let p_target = graph[target_idx].position;

            let delta = [p_target[0] - p_source[0], p_target[1] - p_source[1]];
            let distance = (delta[0].powi(2) + delta[1].powi(2)).sqrt();

            if distance > 0.0 {
                let force = distance.powi(2) / k;
                let displacement = [
                    delta[0] / distance * force,
                    delta[1] / distance * force,
                ];
                displacements.get_mut(&source_idx).unwrap()[0] += displacement[0];
                displacements.get_mut(&source_idx).unwrap()[1] += displacement[1];
                displacements.get_mut(&target_idx).unwrap()[0] -= displacement[0];
                displacements.get_mut(&target_idx).unwrap()[1] -= displacement[1];
            }
        }

        // Apply displacements and cool down
        for node_idx in graph.node_indices() {
            let disp = displacements[&node_idx];
            let disp_len = (disp[0].powi(2) + disp[1].powi(2)).sqrt();

            if disp_len > 0.0 {
                graph[node_idx].position[0] += disp[0] / disp_len * disp_len.min(c);
                graph[node_idx].position[1] += disp[1] / disp_len * disp_len.min(c);
            }
        }
    }
} 