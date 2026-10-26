// 3619. Count Islands With Total Value Divisible by K
// https://leetcode.com/problems/count-islands-with-total-value-divisible-by-k/

struct Solution;

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_islands_with_total_value_divisible_by_k::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_count_islands_1() {
        let grid = to_vec2d([
            [0, 2, 1, 0, 0],
            [0, 5, 0, 0, 5],
            [0, 0, 1, 0, 0],
            [0, 1, 4, 7, 0],
            [0, 2, 0, 0, 8],
        ]);
        let k = 5;
        assert_eq!(2, Solution::count_islands(grid, k));
    }

    #[test]
    fn test_count_islands_2() {
        let grid = to_vec2d([[3, 0, 3, 0], [0, 3, 0, 3], [3, 0, 3, 0]]);
        let k = 3;
        assert_eq!(6, Solution::count_islands(grid, k));
    }
}
