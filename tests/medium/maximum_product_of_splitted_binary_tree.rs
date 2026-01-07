// 1339. Maximum Product of Splitted Binary Tree
// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/

struct Solution;

use crate::shared::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // First pass: calculate total sum of all nodes
        fn calculate_total(node: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    n.val as i64 + calculate_total(&n.left) + calculate_total(&n.right)
                }
            }
        }

        // Second pass: find maximum product by trying each possible split
        fn find_max_product(
            node: &Option<Rc<RefCell<TreeNode>>>,
            total: i64,
            max_product: &mut i64,
        ) -> i64 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left_sum = find_max_product(&n.left, total, max_product);
                    let right_sum = find_max_product(&n.right, total, max_product);
                    let subtree_sum = n.val as i64 + left_sum + right_sum;

                    // Try splitting at this node (removing edge to parent)
                    // This creates subtree with sum=subtree_sum and another with sum=(total-subtree_sum)
                    let product = subtree_sum * (total - subtree_sum);
                    *max_product = (*max_product).max(product);

                    subtree_sum
                }
            }
        }

        let total = calculate_total(&root);
        let mut max_product = 0i64;
        find_max_product(&root, total, &mut max_product);

        (max_product % MOD) as i32
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
