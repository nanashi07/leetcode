// # 2322. Minimum Score After Removals on a Tree
// https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/

struct Solution;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_score_after_removals_on_a_tree::Solution;

    #[test]
    fn test_minimum_score_1() {
        let nums = [1, 5, 5, 4, 11].to_vec();
        let edges = [[0, 1], [1, 2], [1, 3], [3, 4]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(9, Solution::minimum_score(nums, edges));
    }

    #[test]
    fn test_minimum_score_2() {
        let nums = [5, 5, 2, 4, 4, 2].to_vec();
        let edges = [[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(0, Solution::minimum_score(nums, edges));
    }
}
