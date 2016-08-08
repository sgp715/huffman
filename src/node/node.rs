pub use utils::utils::*;
pub use petgraph::Graph;
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


pub fn fuse_nodes(gr: &mut Graph<String, String>, dict: &HashMap<String, f32>) -> HashMap<String, f32> {
    
    let mut new_dict: HashMap<String, f32> = dict.clone();
    
    if new_dict.len() <= 1 {
        assert(false);
    }
    
    // remove two letters
    let first_letter = minimum(&new_dict);
    let mut new_prob: f32;
    {
        let first_prob = new_dict.get(&first_letter).unwrap();
        new_prob =  *first_prob;
    }
    new_dict.remove(&first_letter);
    
    let second_letter = minimum(&new_dict);
    {
        let second_prob = new_dict.get(&second_letter).unwrap();
        new_prob = new_prob + *second_prob;
    }
    new_dict.remove(&second_letter);
    let new_letter = first_letter + &second_letter;
    
    // create the new entry
    new_dict.insert(new_letter, new_prob);
    
    new_dict

}



