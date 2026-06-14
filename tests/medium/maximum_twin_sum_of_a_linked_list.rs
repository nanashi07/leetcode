// 2130. Maximum Twin Sum of a Linked List
// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/

use crate::shared::list_node_box::ListNode;

struct Solution;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut vals = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            vals.push(node.val);
            cur = &node.next;
        }
        let n = vals.len();
        let mut max = 0;
        for i in 0..n / 2 {
            max = max.max(vals[i] + vals[n - 1 - i]);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_twin_sum_of_a_linked_list::Solution;
    use crate::shared::list_node_box::create_node_list;

    #[test]
    fn test_pair_sum_1() {
        let head = create_node_list(&[5, 4, 2, 1]);
        assert_eq!(6, Solution::pair_sum(head));
    }

    #[test]
    fn test_pair_sum_2() {
        let head = create_node_list(&[4, 2, 2, 3]);
        assert_eq!(7, Solution::pair_sum(head));
    }

    #[test]
    fn test_pair_sum_3() {
        let head = create_node_list(&[1, 100000]);
        assert_eq!(100001, Solution::pair_sum(head));
    }
}
