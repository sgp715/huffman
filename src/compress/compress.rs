use node::node::*;
use std::collections::HashMap;

/*
pub fn fuse(dict: &HashMap<String, f32>) -> HashMap<String, f32> {
    //! takes in a hash map and adds the probability of the minimum two
    //! and returns a new hash map with the combined values
    
    let mut new_dict: HashMap<String, f32> = dict.clone();
    
    if new_dict.len() <= 1 {
        return new_dict;
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
*/


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


pub fn create_tree(s: &str) -> Graph<String, String> {
    //! given a string create the huffman tree that will be used to encode and decode stirngs
    
    let mut graph = Graph::<String, String>::default();
    let mut dict = initialize_node_dictionary(&mut graph, s);
    
    
    // loop until there is only one element left in the dictionary
    // this corresponds to the root node of the tree
    while dict.len() > 1 {
    
        fuse_nodes(&mut graph, &mut dict);
    }
    
    graph

}

pub fn print_graph(g: Graph<String, String>) {



}
