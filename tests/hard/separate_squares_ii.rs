// 3454. Separate Squares II
// https://leetcode.com/problems/separate-squares-ii/

struct Solution;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::separate_squares_ii::Solution;

    #[test]
    fn test_separate_squares_1() {
        let squares = [[0, 0, 1], [2, 2, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1.00000, Solution::separate_squares(squares));
    }

    #[test]
    fn test_separate_squares_2() {
        let squares = [[0, 0, 2], [1, 1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1.00000, Solution::separate_squares(squares));
    }
}
