// 1161. Maximum Level Sum of a Binary Tree
// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/

struct Solution;

use crate::shared::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;

        if root.is_none() {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut max_sum = i32::MIN;
        let mut max_level = 1;
        let mut current_level = 1;

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level_sum = 0;

            for _ in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();
                    level_sum += node.val;

                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }

            if level_sum > max_sum {
                max_sum = level_sum;
                max_level = current_level;
            }

            current_level += 1;
        }

        max_level
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_level_sum_of_a_binary_tree::Solution;
    use crate::shared::tree_node::create_tree;

    #[test]
    fn test_max_level_sum_1() {
        // Example 1: [1,7,0,7,-8,null,null]
        // Tree structure:
        //        1
        //       / \
        //      7   0
        //     / \
        //    7  -8
        // Level 1: 1, Level 2: 7+0=7, Level 3: 7+(-8)=-1
        // Maximum sum is at level 2
        let root = create_tree(&[Some(1), Some(7), Some(0), Some(7), Some(-8), None, None]);
        assert_eq!(2, Solution::max_level_sum(root));
    }

    #[test]
    fn test_max_level_sum_2() {
        // Example 2: [989,null,10250,98693,-89388,null,null,null,-32127]
        // Tree structure:
        //           989
        //             \
        //            10250
        //            /    \
        //        98693   -89388
        //                    \
        //                   -32127
        // Level 1: 989, Level 2: 10250, Level 3: 98693+(-89388)=9305, Level 4: -32127
        // Maximum sum is at level 2
        let root = create_tree(&[
            Some(989),
            None,
            Some(10250),
            Some(98693),
            Some(-89388),
            None,
            None,
            None,
            Some(-32127),
        ]);
        assert_eq!(2, Solution::max_level_sum(root));
    }

    #[test]
    fn test_max_level_sum_single_node() {
        // Single node tree
        let root = create_tree(&[Some(1)]);
        assert_eq!(1, Solution::max_level_sum(root));
    }

    #[test]
    fn test_max_level_sum_negative_values() {
        // Tree with all negative values
        let root = create_tree(&[Some(-1), Some(-2), Some(-3)]);
        assert_eq!(1, Solution::max_level_sum(root));
    }
}
