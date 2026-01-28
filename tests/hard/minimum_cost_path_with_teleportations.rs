// 3651. Minimum Cost Path with Teleportations
// https://leetcode.com/problems/minimum-cost-path-with-teleportations/

struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_cost_path_with_teleportations::Solution;

    #[test]
    fn test_min_cost_1() {
        let grid = [[1, 3, 3], [2, 5, 4], [4, 3, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 2;
        assert_eq!(7, Solution::min_cost(grid, k));
    }

    #[test]
    fn test_min_cost_2() {
        let grid = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 1;
        assert_eq!(9, Solution::min_cost(grid, k));
    }
}
