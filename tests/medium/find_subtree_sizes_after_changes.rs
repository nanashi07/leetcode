// 3331. Find Subtree Sizes After Changes
// https://leetcode.com/problems/find-subtree-sizes-after-changes/

struct Solution;

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_subtree_sizes_after_changes::Solution;

    #[test]
    fn test_find_subtree_sizes_1() {
        let parent: Vec<i32> = vec![-1, 0, 0, 1, 1, 1];
        let s: String = "abaabc".to_string();
        assert_eq!(
            [6, 3, 1, 1, 1, 1].to_vec(),
            Solution::find_subtree_sizes(parent, s)
        );
    }

    #[test]
    fn test_find_subtree_sizes_2() {
        let parent: Vec<i32> = vec![-1, 0, 4, 0, 1];
        let s: String = "abbba".to_string();
        assert_eq!(
            [5, 2, 1, 1, 1].to_vec(),
            Solution::find_subtree_sizes(parent, s)
        );
    }
}
