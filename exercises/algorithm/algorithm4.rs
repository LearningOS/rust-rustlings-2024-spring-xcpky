/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn __insert(node: &mut TreeNode<T>, value: T) {
        if value < node.value {
            match &mut node.left {
                None => node.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut left_node) => Self::__insert(left_node, value),
            }
        } else if value > node.value {
            match &mut node.right {
                None => node.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut right_node) => Self::__insert(right_node, value),
            }
        }
    }
    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match &mut self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }

            Some(ref mut root_box) => {
                Self::__insert(root_box, value);
            }
        }
    }

    fn __search(node_opt: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match node_opt {
            None => false,
            Some(node) => {
                if value < node.value {
                    Self::__search(&node.left, value)
                } else if value > node.value {
                    Self::__search(&node.right, value)
                } else {
                    true
                }
            }
        }
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        Self::__search(&self.root, value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
