#![cfg(test)]

use std::collections::HashMap;

pub use petgraph::Graph;
#[test]
pub fn test_create_graph(){
    
    let mut gr = Graph::new();
    let a = gr.add_node(0.);
    let b = gr.add_node(0.);
    let c = gr.add_node(0.);
    gr.add_edge(a, b, 3.);
    gr.add_edge(b, c, 2.);
    gr.add_edge(c, b, 1.);    

}

#[test]
pub fn test_add(){

    let mut gr = Graph::<Option<String>, bool>::default();
    let a = gr.add_node(None);
    let b = gr.add_node(Some("m".to_string()));
    let c = gr.add_node(Some("e".to_string()));
    gr.add_edge(a, b, true);
    gr.add_edge(a, c, true);

}
