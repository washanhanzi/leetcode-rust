use crate::n21::entity::ListNode;

//https://leetcode.com/problems/merge-two-sorted-lists/
fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1 == None && list2 == None {
        return None;
    }
    if list1 == None {
        return list2;
    }
    if list2 == None {
        return list1;
    }

    let mut head: ListNode;
    if list1.as_deref().unwrap().val < list2.as_deref().unwrap().val {
        head = ListNode::new(list1.as_deref().unwrap().val);
        recursive(&mut head, list1.unwrap().next, list2);
    } else {
        head = ListNode::new(list2.as_deref().unwrap().val);
        recursive(&mut head, list1, list2.unwrap().next);
    }
    Some(Box::new(head))
}

fn recursive(head: &mut ListNode, cur1: Option<Box<ListNode>>, cur2: Option<Box<ListNode>>) {
    if cur1 == None && cur2 == None {
        return;
    }
    if cur1 == None {
        head.next = cur2;
        return;
    }
    if cur2 == None {
        head.next = cur1;
        return;
    }
    if cur1.as_deref().unwrap().val < cur2.as_deref().unwrap().val {
        head.next = Some(Box::new(ListNode::new(cur1.as_deref().unwrap().val)));
        recursive(head.next.as_mut().unwrap(), cur1.unwrap().next, cur2);
    } else {
        head.next = Some(Box::new(ListNode::new(cur2.as_deref().unwrap().val)));
        recursive(head.next.as_mut().unwrap(), cur1, cur2.unwrap().next);
    }
}

//copied, a more elegant solution
pub fn merge_two_lists_elegant(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1_cur = list1;
    let mut l2_cur = list2;
    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut cur = &mut dummy_head;
    loop {
        match (&l1_cur, &l2_cur) {
            (Some(v1), Some(v2)) => {
                if v1.val < v2.val {
                    cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(v1.val)));
                    cur = &mut cur.as_mut().unwrap().next;
                    l1_cur = l1_cur.unwrap().next;
                } else {
                    cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(v2.val)));
                    cur = &mut cur.as_mut().unwrap().next;
                    l2_cur = l2_cur.unwrap().next;
                }
            }
            (Some(v1), None) => {
                cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(v1.val)));
                cur = &mut cur.as_mut().unwrap().next;
                l1_cur = l1_cur.unwrap().next;
            }
            (None, Some(v2)) => {
                cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(v2.val)));
                cur = &mut cur.as_mut().unwrap().next;
                l2_cur = l2_cur.unwrap().next;
            }
            (None, None) => break,
        }
    }
    dummy_head.unwrap().next
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::n21::entity::compare;
    struct TestData {
        name: String,
        input: [Option<Box<ListNode>>; 2],
        output: Option<Box<ListNode>>,
    }

    #[test]
    fn run_test() {
        let data = vec![
            TestData {
                name: String::from("case1"),
                input: [
                    ListNode::from_arr(vec![1, 2, 4]),
                    ListNode::from_arr(vec![1, 3, 4]),
                ],
                output: ListNode::from_arr(vec![1, 1, 2, 3, 4, 4]),
            },
            TestData {
                name: String::from("case2"),
                input: [None, None],
                output: None,
            },
            TestData {
                name: String::from("case3"),
                input: [None, ListNode::from_arr(vec![0])],
                output: ListNode::from_arr(vec![0]),
            },
        ];
        for d in data {
            let [list1, list2] = d.input;
            let head = merge_two_lists(list1.clone(), list2.clone());
            assert_eq!(
                compare(&head, &d.output),
                true,
                "test case failed: {}",
                d.name
            );
            let head2 = merge_two_lists_elegant(list1.clone(), list2.clone());
            assert_eq!(
                compare(&head2, &d.output),
                true,
                "test case failed: {}",
                d.name
            );
        }
    }
}
