use anyhow::{Error, Result};
use bst::{BSTPtr, BST};

pub mod bst;

fn main() -> Result<(), Error> {
    let keys: [i64; 6] = [6, 5, 2, 5, 7, 8];

    let mut tree: BSTPtr = None;
    for key in keys {
        tree = BST::insert(tree, key);
    }

    let tree_elements = BST::to_list(tree);
    dbg!(tree_elements);

    Ok(())
}
