// 3548. Equal Sum Grid Partition II
// https://leetcode.com/problems/equal-sum-grid-partition-ii/

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::equal_sum_grid_partition_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_can_partition_grid_1() {
        let grid = to_vec2d([[1, 4], [2, 3]]);
        assert_eq!(true, Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_can_partition_grid_2() {
        let grid = to_vec2d([[1, 2], [3, 4]]);
        assert_eq!(true, Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_can_partition_grid_3() {
        let grid = to_vec2d([[1, 2, 4], [2, 3, 5]]);
        assert_eq!(false, Solution::can_partition_grid(grid));
    }
}
