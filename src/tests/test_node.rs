#![cfg(test)]
use node::*;


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


#[test]
pub fn test_find_node(){

    let dict = create_probability_dictionary("Hello");

    let tree_tuple = create_tree(&dict);

    let node_index = find_node(&tree_tuple, "H");

    assert!(node_index.is_some());

}

#[test]
pub fn test_encode_and_decode(){

    let s = "Hello";

    let dict = create_probability_dictionary(s);

    let tree_tuple = create_tree(&dict);

    let b = encode_string(&tree_tuple, "H");
    let H = decode_string(&tree_tuple, &b);
    assert_eq!("H".to_string(), H);

    let mut b = encode_string(&tree_tuple, "e");
    let e = decode_string(&tree_tuple, &b);
    assert_eq!("e".to_string(), e);

    let b = encode_string(&tree_tuple, "l");
    let l = decode_string(&tree_tuple, &b);
    assert_eq!("l".to_string(), l);

    let b = encode_string(&tree_tuple, "l");
    let l = decode_string(&tree_tuple, &b);
    assert_eq!("l".to_string(), l);

    let b = encode_string(&tree_tuple, s);
    let Hello = decode_string(&tree_tuple, &b);
    assert_eq!(s.to_string(), Hello);

    let ss = "Hey what's up \n";
    let ss_dict = create_probability_dictionary(ss);
    let tree_tuple = create_tree(&ss_dict);
    let b = encode_string(&tree_tuple, ss);
    let hey = decode_string(&tree_tuple, &b);
    assert_eq!(ss.to_string(), hey);


}
