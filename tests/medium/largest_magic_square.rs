// 1895. Largest Magic Square
// https://leetcode.com/problems/largest-magic-square/

struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use crate::medium::largest_magic_square::Solution;

    #[test]
    fn test_largest_magic_square_1() {
        let grid = [
            [7, 1, 4, 5, 6],
            [2, 5, 1, 6, 4],
            [1, 5, 4, 3, 2],
            [1, 2, 7, 3, 4],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(3, Solution::largest_magic_square(grid));
    }

    #[test]
    fn test_largest_magic_square_2() {
        let grid = [[5, 1, 3, 1], [9, 3, 3, 1], [1, 3, 3, 8]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::largest_magic_square(grid));
    }
}
