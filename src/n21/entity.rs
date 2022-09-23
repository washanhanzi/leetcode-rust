// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    pub fn from_arr(val: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(val[0]);
        let mut cur = &mut head;
        for i in 1..val.len() {
            cur.next = Some(Box::new(ListNode::new(val[i])));
            cur = cur.next.as_mut().unwrap();
        }
        Some(Box::new(head))
    }
}

pub fn compare(list1: &Option<Box<ListNode>>, list2: &Option<Box<ListNode>>) -> bool {
    if list1 == &None && list2 == &None {
        return true;
    }
    if list1 == &None || list2 == &None {
        return false;
    }
    if list1.as_deref().unwrap().val != list2.as_deref().unwrap().val {
        return false;
    }
    return compare(
        &list1.as_deref().unwrap().next,
        &list2.as_deref().unwrap().next,
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_arr_test() {
        let input = vec![1, 2, 3, 4];
        let output = ListNode::from_arr(input.clone());
        assert_eq!(output.as_deref().unwrap().val, input[0], "first ele");
        let mut cur = output.unwrap().next;
        for i in 1..input.len() {
            assert_eq!(cur.as_deref().unwrap().val, input[i], "{}th ele", i);
            cur = cur.unwrap().next;
        }
    }

    struct TestCompareDatum {
        name: String,
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
        output: bool,
    }

    #[test]
    fn compare_test() {
        let data = [
            TestCompareDatum {
                name: String::from("empty list 1"),
                list1: None,
                list2: ListNode::from_arr(vec![1, 2, 3]),
                output: false,
            },
            TestCompareDatum {
                name: String::from("empty list 2"),
                list2: None,
                list1: ListNode::from_arr(vec![1, 2, 3]),
                output: false,
            },
            TestCompareDatum {
                name: String::from("length not equal"),
                list2: ListNode::from_arr(vec![1, 2, 3, 4]),
                list1: ListNode::from_arr(vec![1, 2, 3]),
                output: false,
            },
            TestCompareDatum {
                name: String::from("value not equal"),
                list2: ListNode::from_arr(vec![1, 2, 4]),
                list1: ListNode::from_arr(vec![1, 2, 3]),
                output: false,
            },
            TestCompareDatum {
                name: String::from("normal equal"),
                list1: ListNode::from_arr(vec![1, 2, 3]),
                list2: ListNode::from_arr(vec![1, 2, 3]),
                output: true,
            },
        ];
        for d in data {
            assert_eq!(
                compare(&d.list1, &d.list2),
                d.output,
                "test case failed {}",
                d.name
            );
        }
    }
}
