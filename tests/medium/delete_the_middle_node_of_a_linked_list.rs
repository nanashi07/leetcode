// 2095. Delete the Middle Node of a Linked List
// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/

use crate::shared::list_node_box::ListNode;

struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut slow: *mut Box<ListNode> = &mut dummy;
        let mut fast: *const Option<Box<ListNode>> = unsafe { &(*slow).next };
        unsafe {
            while (*fast).is_some() && (*fast).as_ref().unwrap().next.is_some() {
                slow = (*slow).next.as_mut().unwrap();
                fast = &(*fast).as_ref().unwrap().next.as_ref().unwrap().next;
            }
            let next = (*slow).next.take();
            (*slow).next = next.unwrap().next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::delete_the_middle_node_of_a_linked_list::Solution;
    use crate::shared::list_node_box::create_node_list;

    #[test]
    fn test_delete_middle_1() {
        let head = create_node_list(&[1, 3, 4, 7, 1, 2, 6]);
        let output = create_node_list(&[1, 3, 4, 1, 2, 6]);
        assert_eq!(output, Solution::delete_middle(head));
    }

    #[test]
    fn test_delete_middle_2() {
        let head = create_node_list(&[1, 2, 3, 4]);
        let output = create_node_list(&[1, 2, 4]);
        assert_eq!(output, Solution::delete_middle(head));
    }

    #[test]
    fn test_delete_middle_3() {
        let head = create_node_list(&[2, 1]);
        let output = create_node_list(&[2]);
        assert_eq!(output, Solution::delete_middle(head));
    }
}
