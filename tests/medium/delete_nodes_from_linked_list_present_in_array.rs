// # 3217. Delete Nodes From Linked List Present in Array
// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/

use crate::shared::ListNode;
use std::collections::HashSet;

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
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        println!("nums: {:?}, head: {:?}", &nums, &head);

        if head.is_none() {
            return head;
        }

        let set = nums.into_iter().collect::<HashSet<i32>>();

        let mut node = &head;
        let mut first: Option<Box<ListNode>> = None;

        // find first one
        while let Some(curr) = node {
            if !set.contains(&curr.val) {
                first = Some(Box::new(ListNode::new(curr.val)));
                node = &curr.next;
                break;
            }
            node = &curr.next;
        }

        let mut tail = &mut first;
        // continue fill
        while let Some(curr) = node {
            if !set.contains(&curr.val) {
                if let Some(ref mut tail_value) = tail {
                    tail_value.next = Some(Box::new(ListNode::new(curr.val)));
                    tail = &mut tail_value.next;
                }
            }
            node = &curr.next;
        }

        first
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::delete_nodes_from_linked_list_present_in_array::Solution;
    use crate::shared::create_node_list;

    #[test]
    fn test_modified_list_1() {
        let nums = [1, 2, 3].to_vec();
        let head = create_node_list(&[1, 2, 3, 4, 5]);
        let output = create_node_list(&[4, 5]);
        assert_eq!(output, Solution::modified_list(nums, head));
    }

    #[test]
    fn test_modified_list_2() {
        let nums = [1].to_vec();
        let head = create_node_list(&[1, 2, 1, 2, 1, 2]);
        let output = create_node_list(&[2, 2, 2]);
        assert_eq!(output, Solution::modified_list(nums, head));
    }

    #[test]
    fn test_modified_list_3() {
        let nums = [5].to_vec();
        let head = create_node_list(&[1, 2, 3, 4]);
        let output = create_node_list(&[1, 2, 3, 4]);
        assert_eq!(output, Solution::modified_list(nums, head));
    }
}
