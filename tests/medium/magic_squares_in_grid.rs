// 840. Magic Squares In Grid
// https://leetcode.com/problems/magic-squares-in-grid/

struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::magic_squares_in_grid::Solution;

    #[test]
    fn test_num_magic_squares_inside_1() {
        let grid = [[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::num_magic_squares_inside(grid));
    }

    #[test]
    fn test_num_magic_squares_inside_2() {
        let grid = [[8]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(0, Solution::num_magic_squares_inside(grid));
    }
}
