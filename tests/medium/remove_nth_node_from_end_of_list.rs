// # 19. Remove Nth Node From End of List
// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut current = &head;
        let mut len = 0;

        while let Some(node) = current {
            len = len + 1;
            if node.next.is_none() {
                // last one
                break;
            } else {
                current = &node.next;
            }
        }

        std::mem::drop(current);

        let pos = len - n;

        // read option usage from
        // https://chungchris.github.io/2020/07/09/software/leetcode/Remove-Nth-Node-From-End-of-List/
        if pos == 0 {
            head.unwrap().next
        } else if pos > 0 {
            // only mut var can be live as mut ref?
            let mut head = head;
            let mut current = &mut head;
            for _ in 1..pos {
                current = &mut (current.as_mut().unwrap().next);
            }

            let node = current.as_mut().unwrap();
            node.next = node.next.as_mut().unwrap().next.take();

            head
        } else {
            None
        }
    }

    fn create_node_list(values: &[i32]) -> Option<Box<ListNode>> {
        if values.is_empty() {
            return None;
        }

        let mut nodes: Vec<Box<ListNode>> = values
            .iter()
            .rev()
            .map(|v| Box::new(ListNode::new(*v)))
            .collect();

        if nodes.len() == 1 {
            return Some(nodes[0].clone());
        }

        let result = nodes.iter_mut().reduce(|last, item| {
            item.next = Some(Box::clone(last));
            item
        });

        Some(result.unwrap().clone())
    }
}

#[test]
fn test_remove_nth_from_end() {
    let head = Solution::create_node_list(&[1, 2, 3, 4, 5]);
    let n = 2;
    let result = Solution::remove_nth_from_end(head, n);
    let expected = Solution::create_node_list(&[1, 2, 3, 5]);
    assert_eq!(expected, result, "first");

    let head = Solution::create_node_list(&[1]);
    let n = 1;
    let result = Solution::remove_nth_from_end(head, n);
    let expected = Solution::create_node_list(&[]);
    assert_eq!(expected, result, "second");

    let head = Solution::create_node_list(&[1, 2]);
    let n = 1;
    let result = Solution::remove_nth_from_end(head, n);
    let expected = Solution::create_node_list(&[1]);
    assert_eq!(expected, result, "third");
}
