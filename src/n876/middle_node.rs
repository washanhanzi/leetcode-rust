//https://leetcode.com/problems/middle-of-the-linked-list/
// Definition for singly-linked list.
use crate::n21::entity::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    }
    if head.as_ref().unwrap().next == None {
        return head;
    }
    let mut mid: Option<Box<ListNode>> = None;
    let mut cur = head;
    let mut count = 1;
    loop {
        match cur.as_ref() {
            Some(v) => {
                //the above code already dealt with this condition
                if count == 1 {
                    count += 1;
                    cur = cur.unwrap().next;
                    continue;
                }
                if count == 2 {
                    mid = Some(v.clone());
                    count += 1;
                    cur = cur.unwrap().next;
                    continue;
                }
                if count % 2 == 0 {
                    mid = mid.unwrap().next;
                    count += 1;
                    cur = cur.unwrap().next;
                } else {
                    count += 1;
                    cur = cur.unwrap().next;
                }
            }
            None => return mid,
        }
    }
}

fn middle_node_elegant(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut t: &Option<Box<ListNode>> = &head;
    let mut h: &Option<Box<ListNode>> = &head;
    while h.is_some() && (&h.as_ref().unwrap().next).is_some() {
        t = &t.as_ref().unwrap().next;
        h = &(&h.as_ref().unwrap().next).as_ref().unwrap().next;
    }
    t.clone()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::n21::entity::{compare, ListNode};

    #[test]
    fn case1() {
        let input = ListNode::from_arr(vec![1, 2, 3, 4, 5]);
        assert_eq!(
            compare(&middle_node(input), &ListNode::from_arr(vec![3, 4, 5])),
            true
        )
    }
}
