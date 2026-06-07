// 2196. Create Binary Tree From Descriptions
// https://leetcode.com/problems/create-binary-tree-from-descriptions/

struct Solution;

use crate::shared::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::{HashMap, HashSet};

        let mut nodes = HashMap::with_capacity(descriptions.len() * 2);
        let mut children = HashSet::with_capacity(descriptions.len());

        for description in descriptions {
            let parent = description[0];
            let child = description[1];
            let is_left = description[2] == 1;

            let parent_node = Rc::clone(
                nodes
                    .entry(parent)
                    .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent)))),
            );
            let child_node = Rc::clone(
                nodes
                    .entry(child)
                    .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child)))),
            );

            if is_left {
                parent_node.borrow_mut().left = Some(child_node);
            } else {
                parent_node.borrow_mut().right = Some(child_node);
            }

            children.insert(child);
        }

        nodes
            .into_iter()
            .find_map(|(value, node)| (!children.contains(&value)).then_some(node))
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::create_binary_tree_from_descriptions::Solution;
    use crate::shared::tree_node::create_tree;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_create_binary_tree_1() {
        let descriptions = to_vec2d([
            [20, 15, 1],
            [20, 17, 0],
            [50, 20, 1],
            [50, 80, 0],
            [80, 19, 1],
        ]);
        let output = create_tree(&[Some(50), Some(20), Some(80), Some(15), Some(17), Some(19)]);
        assert_eq!(output, Solution::create_binary_tree(descriptions));
    }

    #[test]
    fn test_create_binary_tree_2() {
        let descriptions = to_vec2d([[1, 2, 1], [2, 3, 0], [3, 4, 1]]);
        let output = create_tree(&[Some(1), Some(2), None, None, Some(3), Some(4)]);
        assert_eq!(output, Solution::create_binary_tree(descriptions));
    }
}
