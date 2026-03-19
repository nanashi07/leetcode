// 3212. Count Submatrices With Equal Frequency of X and Y
// https://leetcode.com/problems/count-submatrices-with-equal-frequency-of-x-and-y/

struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_submatrices_with_equal_frequency_of_x_and_y::Solution;

    #[test]
    fn test_number_of_submatrices_1() {
        let grid = [["X", "Y", "."], ["Y", ".", "."]]
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn test_number_of_submatrices_2() {
        let grid = [["X", "X"], ["X", "Y"]]
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn test_number_of_submatrices_3() {
        let grid = [[".", "."], [".", "."]]
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }
}
