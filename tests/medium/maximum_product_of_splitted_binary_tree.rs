// 1339. Maximum Product of Splitted Binary Tree
// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/

struct Solution;

use crate::shared::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_product_of_splitted_binary_tree::Solution;
    use crate::shared::create_tree;

    #[test]
    fn test_max_product_1() {
        let root = create_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        assert_eq!(110, Solution::max_product(root));
    }

    #[test]
    fn test_max_product_2() {
        let root = create_tree(&[
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
            Some(6),
        ]);
        assert_eq!(90, Solution::max_product(root));
    }
}
