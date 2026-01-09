// 865. Smallest Subtree with all the Deepest Nodes
// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/

struct Solution;

use crate::shared::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::create_tree;

    #[test]
    fn test_subtree_with_all_deepest_1() {
        let root = create_tree(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let output = create_tree(&[Some(2), Some(7), Some(4)]);
        assert_eq!(output, root);
    }

    #[test]
    fn test_subtree_with_all_deepest_2() {
        let root = create_tree(&[Some(1)]);
        let output = create_tree(&[Some(1)]);
        assert_eq!(output, root);
    }

    #[test]
    fn test_subtree_with_all_deepest_3() {
        let root = create_tree(&[Some(0), Some(1), Some(3), None, Some(2)]);
        let output = create_tree(&[Some(2)]);
        assert_eq!(output, root);
    }
}
