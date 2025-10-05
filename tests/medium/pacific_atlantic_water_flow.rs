// # 417. Pacific Atlantic Water Flow
// https://leetcode.com/problems/pacific-atlantic-water-flow/

struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::pacific_atlantic_water_flow::Solution;

    #[test]
    fn test_pacific_atlantic_1() {
        let heights = [
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let output = [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::pacific_atlantic(heights));
    }

    #[test]
    fn test_pacific_atlantic_2() {
        let heights = [[1]].into_iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let output = [[0, 0]].into_iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(output, Solution::pacific_atlantic(heights));
    }
}
