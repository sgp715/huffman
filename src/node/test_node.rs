#![cfg(test)]
use node::node::*;


use std::collections::HashMap;


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

/*
#[test]
pub fn test_initialize_node_dictionary(){

    let mut graph = Graph::<String, String>::default();
    let actual = initialize_node_dictionary(&mut graph, "hello");
    let mut node_dict: HashMap<NodeIndex, f32> = HashMap::new();
    let mut gr = Graph::<String, String>::default();
    let h = gr.add_node("h".to_string());
    node_dict.insert(h, 0.2);
    let e = gr.add_node("e".to_string());
    node_dict.insert(e, 0.2);
    let l = gr.add_node("l".to_string());
    node_dict.insert(l, 0.4);
    let o = gr.add_node("o".to_string());
    node_dict.insert(o, 0.2);
    assert_eq!(actual, node_dict);

}*/

#[test] 
pub fn test_min_node(){

    let mut graph = Graph::<String, String>::default();
    let a = graph.add_node("a".to_string());
    let b = graph.add_node("b".to_string());
    
    let mut node_dict = HashMap::new();
    node_dict.insert(a, 0.1);
    node_dict.insert(b, 0.2);
    
    let actual = min_node(&node_dict);
    
    assert_eq!(actual, a);
    
    
}

#[test]
pub fn test_add_nodes(){

    let mut gr = Graph::<String, String>::default();
    let node0 = gr.add_node("Hey".to_string());
    let node1 = gr.add_node("Ho".to_string());
    let root_node = add_nodes(&mut gr, node0, node1, "root");
    
    let mut equal = true;
    for n in gr.neighbors(root_node){
    
        if n == node0 {
            continue;
        }
        
        if n == node1 {
            continue;
        }
        
        equal = false;
    }
    
    assert_eq!(equal, true);

}

