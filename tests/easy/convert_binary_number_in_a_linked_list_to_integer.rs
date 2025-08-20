// # 1290. Convert Binary Number in a Linked List to Integer
// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/

use crate::shared::ListNode;

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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let mut n = 0;
        while let Some(value) = head {
            n = n << 1;
            n += value.val;
            head = value.next
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::convert_binary_number_in_a_linked_list_to_integer::Solution;
    use crate::shared::create_node_list;

    #[test]
    fn test_get_decimal_value_1() {
        let head = create_node_list(&[1, 0, 1]);
        assert_eq!(5, Solution::get_decimal_value(head));
    }

    #[test]
    fn test_get_decimal_value_2() {
        let head = create_node_list(&[0]);
        assert_eq!(0, Solution::get_decimal_value(head));
    }

    #[test]
    fn test_get_decimal_value_3() {
        let head = create_node_list(&[1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(18880, Solution::get_decimal_value(head));
    }
}
