#![cfg(test)]
use node::node::*;
use compress::compress::*;
use std::collections::HashMap;

/*
#[test]
pub fn test_fuse_nodes(){
    //! only works some times because order on dict is switched

    let mut graph = Graph::<String, String>::default();
    let mut dict = initialize_node_dictionary(&mut graph, "hello");
    fuse_nodes(&mut graph, &mut dict);
    
    let mut weights: HashMap<String, f32> = HashMap::new();
    weights.insert("oh".to_string(), 0.4);
    weights.insert("l".to_string(), 0.4);
    weights.insert("e".to_string(), 0.2);
    
    let mut equal = true;
    for (node_index, prob) in dict {
    
        let weight = graph.node_weight(node_index).expect("could not find node");
        println!("w: {}", weight);
        println!("p: {}", prob);
        
        match weights.get(weight)  {
            Some(p) => if *p == prob { continue },
            None => equal = false
        };
        
        break;
    }
    
    assert_eq!(equal, true);
   
}
*/

#[test]
pub fn test_create_tree(){

    let g0 = create_tree("Hello");
    
    let g1 = create_tree("Hello, World!");
    
    assert_eq!(0, 1);

}
