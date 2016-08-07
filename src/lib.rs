pub mod test;

// holds useful functions for building compression algorithm
extern crate rustc_serialize;
extern crate bincode;
pub mod utils;

// module to serialize data directly into a file
pub mod serialize;

// graph to build tree
extern crate petgraph;
pub mod node;

// does the compressing
pub mod compress;


