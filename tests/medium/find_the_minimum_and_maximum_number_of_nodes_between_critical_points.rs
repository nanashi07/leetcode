// 2058. Find the Minimum and Maximum Number of Nodes Between Critical Points
// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/

use crate::shared::list_node_box::ListNode;

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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_minimum_and_maximum_number_of_nodes_between_critical_points::Solution;
    use crate::shared::list_node_box::create_node_list;

    #[test]
    fn test_nodes_between_critical_points_1() {
        let head = create_node_list(&[3, 1]);
        assert_eq!(
            [-1, -1].to_vec(),
            Solution::nodes_between_critical_points(head)
        );
    }

    #[test]
    fn test_nodes_between_critical_points_2() {
        let head = create_node_list(&[5, 3, 1, 2, 5, 1, 2]);
        assert_eq!(
            [1, 3].to_vec(),
            Solution::nodes_between_critical_points(head)
        );
    }

    #[test]
    fn test_nodes_between_critical_points_3() {
        let head = create_node_list(&[1, 3, 2, 2, 3, 2, 2, 2, 7]);
        assert_eq!(
            [3, 3].to_vec(),
            Solution::nodes_between_critical_points(head)
        );
    }
}
