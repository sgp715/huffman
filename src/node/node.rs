pub use utils::utils::*;
pub use petgraph::{Graph, Bfs};
pub use petgraph::graph::NodeIndex;
use std::collections::HashMap;

pub fn add_nodes(gr: &mut Graph<String, String>, node0: NodeIndex, node1: NodeIndex, new_node_name: &str) -> NodeIndex {
    //! takes in two graph indices and links them and returns a new index
    
   
    let new_node = gr.add_node(new_node_name.to_string());
    
    gr.add_edge(new_node, node0, "0".to_string());
    gr.add_edge(new_node, node1, "1".to_string());
    
    new_node

}

pub fn initialize_node_dictionary(gr: &mut Graph<String, String>, s: &str) -> HashMap<NodeIndex, f32> {

    let mut node_dict: HashMap<NodeIndex, f32> = HashMap::new();

    let string_dict = create_probability_dictionary(s);
    
    for (letter, prob) in string_dict {
        let current_node = gr.add_node(letter);
        node_dict.insert(current_node, prob);
    }
    
    node_dict

}


pub fn min_node(dict: &HashMap<NodeIndex, f32>) -> NodeIndex {
    //! return the minimum node index from the dictionary
    
    let mut minimum_prob: f32 = 1.0f32 / 0.0f32;
    let mut minimum_node: NodeIndex = NodeIndex::new(0);
    for (node, prob) in dict {
    
        if *prob < minimum_prob {
            minimum_prob = *prob;
            minimum_node = *node; // clone
        }
    }
    
    minimum_node
}


pub fn fuse_nodes(graph: &mut Graph<String, String>, dict: &mut HashMap<NodeIndex, f32>) {
    //! fuse the minimum dictionary values as well as connecting the graph

        // let mut new_dict: HashMap<NodeIndex, f32> = dict.clone();
        
        if dict.len() <= 1 {
            panic!("Should not be passing in a dictionary of single value");
        }
        
        let first_node = min_node(&dict);
        let mut first_prob: f32 = 0.0;
        {
            first_prob = *dict.get(&first_node).unwrap();
        }
        dict.remove(&first_node);
        
        let second_node = min_node(&dict);
        let mut second_prob: f32 = 0.0;
        {
            second_prob = *dict.get(&second_node).unwrap();
        }
        dict.remove(&second_node);
        
        let first_weight = graph.node_weight(first_node).expect("could not get string value").clone();
        let second_weight = &graph.node_weight(second_node).expect("could not get string value").clone();
        let new_node_weight = first_weight.to_string() + second_weight;
        let new_node = add_nodes(graph, first_node, second_node, &new_node_weight);
        let new_prob = first_prob + second_prob;
        dict.insert(new_node, new_prob);
        
}


pub fn create_tree(s: &str) -> (Graph<String, String>, NodeIndex) {
    //! given a string create the huffman tree that will be used to encode and decode stirngs
    
    let mut graph = Graph::<String, String>::default();
    let mut dict = initialize_node_dictionary(&mut graph, s);
    
    // loop until there is only one element left in the dictionary
    // this corresponds to the root node of the tree
    while dict.len() > 1 {
    
        fuse_nodes(&mut graph, &mut dict);
    }
    
    // return the first value
    let root_index = match dict.keys().next() {
    
        Some(n) => *n,
        None => panic!("Not able to find root node"),
    };
    
    // return the tuple with the graph and root node
    (graph, root_index)

}




/*
pub fn repr_graph(root: NodeIndex, g: Graph<String, String>) -> String {

    let mut s: String = "".to_string();
    
    for node in g.raw_nodes() {
        let weight = match g.node_weight(node) {
            Option(w) => *w,
            None => panic!("Blank node")
        };  
        s = s + weight;
    }
    
    s

}
*/




