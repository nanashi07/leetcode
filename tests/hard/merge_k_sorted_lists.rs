use crate::shared::{create_node_list, ListNode};

// # 23. Merge k Sorted Lists
// https://leetcode.com/problems/merge-k-sorted-lists/
struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut r: Option<Box<ListNode>> = None;
        let mut rx = &mut r;
        let mut lists = lists;

        while !lists.is_empty() {
            let len = lists.len();
            let mut next: usize = 0;
            let mut min = i32::MAX;
            for i in 0..len {
                let item = &mut lists[i];

                if item.is_some() && min > item.as_mut().unwrap().val {
                    min = item.as_mut().unwrap().val;
                    next = i;
                }
            }

            let item = &mut lists[next];
            if item.is_some() {
                if rx.is_some() {
                    rx.as_mut().unwrap().next = Some(Box::new(ListNode::new(min)));
                    rx = &mut rx.as_mut().unwrap().next;
                } else {
                    r = Some(Box::new(ListNode::new(min)));
                    rx = &mut r;
                }

                lists[next] = item.as_mut().unwrap().next.take();
            } else {
                lists.remove(next);
            }
        }

        r
    }
}

#[test]
fn test_merge_k_lists() {
    let lists = vec![
        create_node_list(&[1, 4, 5]),
        create_node_list(&[1, 3, 4]),
        create_node_list(&[2, 6]),
    ];
    let expected = create_node_list(&[1, 1, 2, 3, 4, 4, 5, 6]);
    let result = Solution::merge_k_lists(lists);
    assert_eq!(expected, result);

    let lists = vec![];
    let expected = create_node_list(&[]);
    let result = Solution::merge_k_lists(lists);
    assert_eq!(expected, result);

    let lists = vec![create_node_list(&[])];
    let expected = create_node_list(&[]);
    let result = Solution::merge_k_lists(lists);
    assert_eq!(expected, result);
}
