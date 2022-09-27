use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut valid = true;
    dfs(root, i64::MIN, i64::MAX, &mut valid);
    return valid;
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64, valid: &mut bool) {
    if let Some(n) = node {
        if let Some(left) = n.borrow().left.clone() {
            if left.borrow().val < n.borrow().val && left.borrow().val as i64 > min {
                dfs(Some(left), min, n.borrow().val as i64, valid);
            } else {
                *valid = false;
            }
        }
        if let Some(right) = n.borrow().right.clone() {
            if right.borrow().val > n.borrow().val && (right.borrow().val as i64) < max {
                dfs(Some(right), n.borrow().val as i64, max, valid);
            } else {
                *valid = false;
            }
        }
    }
}

// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         let mut values = vec![];
//         in_order(&root, &mut values);
//
//         values
//             .iter()
//             .zip(values.iter().skip(1))
//             .all(|(x1, x2)| x1 < x2)
//     }
// }
//
// fn in_order(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
//     if let Some(node) = node {
//         let node_ref = node.borrow();
//         in_order(&node_ref.left, values);
//         values.push(node_ref.val);
//         in_order(&node_ref.right, values);
//     }
// }
