/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::Cursor;


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

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        
        if let Some(ref mut node) = &mut self.root{
            node.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
        
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        
        let mut current = &self.root;
        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Less => {
                    current = &node.left;
                }
                Ordering::Greater => {
                    current = &node.right;
                }
                Ordering::Equal => return true,
            }
        }
        false
    }

    // {
    //     // 使用 `mut current` 作为可变指针，遍历树
    //     let mut current = &self.root;
    //     while let Some(node) = current {
    //         // 安全地解引用 node（假设 node 是一个有效指针）
    //         // 这里 node 是一个 NonNull<Node<T>>
    //         match value.cmp(&(*node).value) {
    //             std::cmp::Ordering::Less => {
    //                 current = &(*node).left;
    //             }
    //             std::cmp::Ordering::Greater => {
    //                 current = &(*node).right;
    //             }
    //             std::cmp::Ordering::Equal => return true,
    //         }
    //     }
    //     false
    // }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        let mut tmp = self;
        loop {
            match value.cmp(&tmp.value) {
                Ordering::Less => {
                    if let Some(ref mut left) = tmp.left {
                        tmp = left;
                    } else {
                        tmp.left = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                }
                Ordering::Greater => {
                    if let Some(ref mut right) = tmp.right {
                        tmp = right;
                    } else {
                        tmp.right = Some(Box::new(TreeNode::new(value)));
                        break;
                    }
                }
                Ordering::Equal => break,
            }
        }
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


