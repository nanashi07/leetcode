// 110. Balanced Binary Tree
// https://leetcode.com/problems/balanced-binary-tree/

struct Solution;

use crate::shared::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_height(&root).is_some()
    }

    // Returns None if tree is unbalanced, otherwise returns the height
    fn check_height(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match node {
            None => Some(0),
            Some(n) => {
                let borrowed = n.borrow();

                // Check left subtree
                let left_height = Self::check_height(&borrowed.left)?;

                // Check right subtree
                let right_height = Self::check_height(&borrowed.right)?;

                // Check if current node is balanced
                if (left_height - right_height).abs() > 1 {
                    return None;
                }

                // Return height of current subtree
                Some(1 + left_height.max(right_height))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::balanced_binary_tree::Solution;
    use crate::shared::create_tree;

    #[test]
    fn test_is_balanced_1() {
        let root = create_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(true, Solution::is_balanced(root));
    }

    #[test]
    fn test_is_balanced_2() {
        let root = create_tree(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert_eq!(false, Solution::is_balanced(root));
    }

    #[test]
    fn test_is_balanced_3() {
        let root = create_tree(&[]);
        assert_eq!(true, Solution::is_balanced(root));
    }
}
