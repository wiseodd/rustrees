pub type BSTPtr = Option<Box<BST>>;

#[derive(Debug)]
pub struct BST {
    key: i64,
    left: BSTPtr,
    right: BSTPtr,
}

impl BST {
    pub fn new(key: i64, left: BSTPtr, right: BSTPtr) -> BSTPtr {
        Some(Box::new(BST { key, left, right }))
    }

    pub fn insert(root: BSTPtr, new_key: i64) -> BSTPtr {
        match root {
            None => BST::new(new_key, None, None),
            Some(node) => {
                if new_key <= node.key {
                    BST::new(node.key, BST::insert(node.left, new_key), node.right)
                } else {
                    BST::new(node.key, node.left, BST::insert(node.right, new_key))
                }
            }
        }
    }

    pub fn to_list(root: BSTPtr) -> Vec<i64> {
        match root {
            None => vec![],
            Some(node) => [
                &BST::to_list(node.left)[..],
                &[node.key],
                &BST::to_list(node.right)[..],
            ]
            .concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_tree() -> BSTPtr {
        let keys: [i64; 6] = [6, 5, 2, 5, 7, 8];

        let mut tree: BSTPtr = None;
        for key in keys {
            tree = Some(BST::insert(tree, key)?);
        }

        tree
    }

    #[test]
    fn test_to_list() {
        let tree = get_tree();
        assert_eq!(BST::to_list(tree), vec![2, 5, 5, 6, 7, 8]);
    }
}
