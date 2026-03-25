// 3546. Equal Sum Grid Partition I
// https://leetcode.com/problems/equal-sum-grid-partition-i/

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::equal_sum_grid_partition_i::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_can_partition_grid_1() {
        let grid = to_vec2d([[1, 4], [2, 3]]);
        assert_eq!(true, Solution::can_partition_grid(grid));
    }
}
