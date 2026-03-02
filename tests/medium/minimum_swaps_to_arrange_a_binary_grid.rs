// 1536. Minimum Swaps to Arrange a Binary Grid
// https://leetcode.com/problems/minimum-swaps-to-arrange-a-binary-grid/

struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_swaps_to_arrange_a_binary_grid::Solution;

    #[test]
    fn test_min_swaps_1() {
        let grid = [[0, 0, 1], [1, 1, 0], [1, 0, 0]]
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::min_swaps(grid));
    }

    #[test]
    fn test_min_swaps_2() {
        let grid = [[0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0]]
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(-1, Solution::min_swaps(grid));
    }

    #[test]
    fn test_min_swaps_3() {
        let grid = [[1, 0, 0], [1, 1, 0], [1, 1, 1]]
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::min_swaps(grid));
    }
}
