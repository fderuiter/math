#![cfg(all(feature = "pure_math"))]

use math_explorer::pure_math::graph_theory::{
    graph::Graph,
    parameters::{degree::degeneracy, modulator::vertex_cover, treewidth::treewidth},
};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

#[test]
fn test_degeneracy_k5() {
    // Create a K5 graph (a clique of 5 vertices).
    // The degeneracy of a K5 graph is 4.
    let mut g: Graph<(), ()> = Graph::new();
    let nodes: Vec<NodeIndex> = (0..5).map(|_| g.add_node(())).collect();

    for i in 0..5 {
        for j in (i + 1)..5 {
            g.add_edge(nodes[i], nodes[j], ());
        }
    }
    assert_eq!(degeneracy(&g), 4);
}

#[test]
fn test_degeneracy_path() {
    // Create a path graph P5: 0 -- 1 -- 2 -- 3 -- 4
    // The degeneracy of a path graph is 1.
    let mut g: Graph<(), ()> = Graph::new();
    let nodes: Vec<NodeIndex> = (0..5).map(|_| g.add_node(())).collect();
    g.add_edge(nodes[0], nodes[1], ());
    g.add_edge(nodes[1], nodes[2], ());
    g.add_edge(nodes[2], nodes[3], ());
    g.add_edge(nodes[3], nodes[4], ());
    assert_eq!(degeneracy(&g), 1);
}

#[test]
fn test_degeneracy_empty() {
    let g: Graph<(), ()> = Graph::new();
    assert_eq!(degeneracy(&g), 0);
}

#[test]
fn test_vertex_cover() {
    // Create a star graph with a center and 3 leaves.
    // The minimum vertex cover is {center}, size 1.
    let mut g: Graph<(), ()> = Graph::new();
    let center = g.add_node(());
    let n1 = g.add_node(());
    let n2 = g.add_node(());
    let n3 = g.add_node(());
    g.add_edge(center, n1, ());
    g.add_edge(center, n2, ());
    g.add_edge(center, n3, ());

    let cover = vertex_cover(&g);

    // Verify that it is a valid vertex cover.
    for edge in g.graph.edge_references() {
        let u = edge.source();
        let v = edge.target();
        assert!(cover.contains(&u) || cover.contains(&v));
    }
}

#[test]
fn test_treewidth_placeholder() {
    let mut g: Graph<(), ()> = Graph::new();
    let n1 = g.add_node(());
    let n2 = g.add_node(());
    g.add_edge(n1, n2, ());
    assert_eq!(treewidth(&g), 0);
}
