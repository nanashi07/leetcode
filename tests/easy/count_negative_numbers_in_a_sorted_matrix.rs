// 1351. Count Negative Numbers in a Sorted Matrix
// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/

struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        println!("grid: {:?}", &grid);

        let mut c = 0;

        for i in 0..grid.len() {
            for j in (0..grid[i].len()).rev() {
                if grid[i][j] >= 0 {
                    break;
                }
                c += 1;
            }
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_negative_numbers_in_a_sorted_matrix::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_count_negatives_1() {
        let grid = to_vec2d([
            [4, 3, 2, -1],
            [3, 2, 1, -1],
            [1, 1, -1, -2],
            [-1, -1, -2, -3],
        ]);
        assert_eq!(8, Solution::count_negatives(grid));
    }

    #[test]
    fn test_count_negatives_2() {
        let grid = to_vec2d([[3, 2], [1, 0]]);
        assert_eq!(0, Solution::count_negatives(grid));
    }
}
