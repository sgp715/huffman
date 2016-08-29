pub use utils::utils::*;
pub use petgraph::{Graph, Bfs, EdgeDirection};
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
    let first_index = match dict.keys().next() {

        Some(n) => *n,
        None => panic!("Not able to find root node"),
    };

    let root_index = graph.add_node("root".to_string());
    graph.add_edge(root_index, first_index, "1".to_string());

    // return the tuple with the graph and root node
    (graph, root_index)

}


pub fn find_node(tree_tuple: &(Graph<String, String>, NodeIndex), s: &str) -> Option<NodeIndex> {
    //! find node index with specific wieght

    let graph = &tree_tuple.0;
    let root_node = tree_tuple.1;
    let mut bfs = Bfs::new(graph, root_node);

    let mut the_node = None;
    loop {

        let node_index: NodeIndex = match bfs.next(graph) {
            Some(n) => n,
            None => break,
        };

        let weight = graph.node_weight(node_index).expect("Could not get weight");
        if weight == s {
            the_node = Some(node_index);
        }
    }

    the_node
}

pub fn encode(tree_tuple: &(Graph<String, String>, NodeIndex), s: &str) -> String {
    //! takes in a letter and traverses the tree backwards and outputs the binary string value

    let starting_index = match find_node(tree_tuple, s) {
        Some(i) => i,
        None => panic!("Could strarting node"),
    };

    let graph = &tree_tuple.0;

    // iterate through all the way to the top
    let mut binary_string: String = "".to_string();
    let mut current_index = starting_index;
    loop {

        // get the parent node (should only be 1)
        let mut node_iter = graph.neighbors_directed(current_index, EdgeDirection::Incoming);
        let mut parent_index = match node_iter.next() {
            Some(pi) => pi,
            None => break,
        };

        // get the wieght of the edge between current node and parent
        let mut edge_index = graph.find_edge(parent_index, current_index).expect("No edge index");
        let mut edge_weight = graph.edge_weight(edge_index).expect("No edge weight");
        binary_string = binary_string + &edge_weight;

        current_index = parent_index;

    }

    reverse_string(&binary_string)

}


pub fn decode(tree_tuple: &(Graph<String, String>, NodeIndex), s: &str) -> String {

    let mut character = "".to_string();
    let graph = &tree_tuple.0;

    let mut current_index = tree_tuple.1;

    /*
    let mut node_iter = graph.neighbors_directed(root_index, EdgeDirection::Outgoing);
    let mut current_index = match node_iter.next() {
        Some(n) => n,
        None => panic!("Empty tree"),
    };
    */

    println!("");

    for c in s.chars() {

        let mut edges = graph.edges_directed(current_index, EdgeDirection::Outgoing);

        let mut found = false;

        loop {

            let child_tuple = match edges.next(){
                Some(ch) => ch,
                None => break,
            };

            let child_index = child_tuple.0;
            let child_weight = child_tuple.1;
            println!("weight: {}", child_weight);
            println!("char: {}", c);
            if &c.to_string() == child_weight {
                current_index = child_index;
                found = true;
            }
        }

        println!("");

        if found == true {
            let current_weight = match graph.node_weight(current_index) {
                Some(w) => w,
                None => panic!("Could weight not assigned")
            };
            println!("{}", current_weight);
            if current_weight.len() == 1 {
                character = character + &current_weight.to_string();
            }
        } else {
            panic!("Could not find matching edge");
        }

    }

    character

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
