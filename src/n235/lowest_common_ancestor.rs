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

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root == None {
        return None;
    }
    let pv = p.unwrap().borrow().val;
    let qv = q.unwrap().borrow().val;
    let mut node = root;
    while let Some(n) = node.clone() {
        //p and q exist in the right subtree
        if pv > n.borrow().val && qv > n.borrow().val {
            node = n.borrow().right.clone();
        } else if pv < n.borrow().val && qv < n.borrow().val {
            //p and q exist in the left subtree
            node = n.borrow().left.clone();
        } else {
            return node;
        }
    }
    None
}
