/*
    ------------------------------------------------------------------------------------
    Module: binary_tree
    File: main.rs
    Description: A Binary Tree capable of construction, searlization, deserializtion.
    Author: Tim Siwula <siwulactim@gmail.com>
    Date: 10/12/2021
    ------------------------------------------------------------------------------------
    Compile: cargo build
        or:  rustc main.rs
    ------------------------------------------------------------------------------------
    Run:  cargo run
     or: ./main
    ------------------------------------------------------------------------------------
    One liner: cargo build && clear && cargo run
    ------------------------------------------------------------------------------------
    Test: cargo test
    ------------------------------------------------------------------------------------
*/

#[macro_use]
extern crate serde_json;

use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

/* BinaryTree enum */
#[derive(Serialize, Deserialize)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<Node<T>>)
}

/* Node struct */
#[derive(Serialize, Deserialize)]
struct Node<T> {
    data: T,
    left_sub_tree: BinaryTree<T>,
    right_sub_tree: BinaryTree<T>
}

/* BinaryTree add method */
impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(Node {
                    data: value,
                    left_sub_tree: BinaryTree::Empty,
                    right_sub_tree: BinaryTree::Empty
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.data {
                    node.left_sub_tree.add(value);
                } else {
                    node.right_sub_tree.add(value);
                }
        }
    }
}


/*
    ------------------------------------------------------------------------------------
    Function:  serialize()
    ------------------------------------------------------------------------------------
    Description:  Builds a BinaryTree struct in memory and then searializes it to disk.
                  Currently uses JSON as the file format.
    ------------------------------------------------------------------------------------
    Input:   BinaryTree struct
    Output:  A File written at ./binary_tree_serialized.json
    ------------------------------------------------------------------------------------
*/
pub fn serialize() -> std::io::Result<()> {

    // create a test BinaryTree struct
    let mut binary_tree = BinaryTree::Empty;

    // add some mock data
    binary_tree.add("Federal");
    binary_tree.add("State");
    binary_tree.add("city");

    // searialize binary_tree and then write as JSON to file
    let serialized: String = serde_json::to_string_pretty(&binary_tree)?;
    fs::write("binary_tree_serialized.json", &serialized).expect("Unable to write file");
    Ok(())
}


/*
    main function
*/
fn main() {

    // Builds a BinaryTree struct in memory and then searializes it to disk
    serialize();

    // Deserialize from disk.
    // Reads file binary_tree_serialized.json and re-constructs in memory.
    let file_path = "./binary_tree_serialized.json";
    let data = fs::read_to_string(file_path).expect("Unable to read the file binary_tree_serialized.json");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    //println!("{}", res)
}






// rust tests

#[cfg(test)]

mod test_one {

    #[test]
    #[should_panic]
    fn test_01() {
        assert!(1 == 1);
        panic!("panic");
    }
}
