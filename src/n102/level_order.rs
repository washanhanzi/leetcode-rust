//https://leetcode.com/problems/binary-tree-level-order-traversal/?envType=study-plan&id=level-1
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //implement a BFS preorder search
    if root == None {
        return vec![];
    }
    //create a vec to store the ready to process nodes
    //all the nodes in this Vec belong to the same level
    //since we don't know the tree height, and we are constantly push values to the vec
    //we are using a VecDeque instead of a regular Vec
    let mut vd = VecDeque::new();
    if let Some(r) = root {
        vd.push_back(r);
    }
    //the result vec
    let mut res: Vec<Vec<i32>> = vec![];
    while !vd.is_empty() {
        let mut level = Vec::new();
        //process all nodes in vd
        for _ in 0..vd.len() {
            if let Some(node) = vd.pop_front() {
                level.push(node.borrow().val);
                //we can mutably borrow and change the Option value to None to avoid clone
                //thanks to https://leetcode.com/problems/binary-tree-level-order-traversal/discuss/1220016/Rust-BFS-solution-with-no-.clone
                //we can also borrow and clone the node
                //node.borrow().left.clone()
                if let Some(left) = node.borrow_mut().left.take() {
                    vd.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    vd.push_back(right);
                }
            }
        }
        res.push(level);
    }
    res
}

// use std::rc::Rc;
// use std::cell::RefCell;
// use std::collections::VecDeque;
// impl Solution {
//     pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
//         let mut rez = vec![];
//         // Using flatten takes care of the edge case of an empty root
//         let mut q = std::iter::once(root).flatten().collect::<VecDeque<_>>();
//         while !q.is_empty() {
//             let n = q.len();
//             // We know the size of the next row, so we can avoid reallocations by using with_capacity
//             let mut row = Vec::with_capacity(n);
//             for _ in 0..n {
//                 let node = q.pop_front().unwrap();
//                 let mut node_ref = node.borrow_mut();
//                 if node_ref.left.is_some() { q.push_back(node_ref.left.take().unwrap()); }
//                 if node_ref.right.is_some() { q.push_back(node_ref.right.take().unwrap()); }
//                 row.push(node_ref.val);
//             }
//             rez.push(row);
//         }
//         rez
//     }
// }
