//https://leetcode.com/problems/reverse-linked-list/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    };
    let mut res: Option<Box<ListNode>> = None;
    let mut cur = head;
    loop {
        match cur {
            Some(v) => {
                res = Some(Box::new(ListNode {
                    val: v.val,
                    next: res,
                }));
                cur = v.next;
            }
            None => return res,
        }
    }
}
