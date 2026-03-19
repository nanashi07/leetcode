// 3212. Count Submatrices With Equal Frequency of X and Y
// https://leetcode.com/problems/count-submatrices-with-equal-frequency-of-x-and-y/

struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        // prefix[i][j] = (count_x, count_y) for submatrix (0,0)-(i-1,j-1)
        let mut px = vec![vec![0i32; n + 1]; m + 1];
        let mut py = vec![vec![0i32; n + 1]; m + 1];
        let mut count = 0;
        for i in 1..=m {
            for j in 1..=n {
                let cx = if grid[i - 1][j - 1] == 'X' { 1 } else { 0 };
                let cy = if grid[i - 1][j - 1] == 'Y' { 1 } else { 0 };
                px[i][j] = px[i - 1][j] + px[i][j - 1] - px[i - 1][j - 1] + cx;
                py[i][j] = py[i - 1][j] + py[i][j - 1] - py[i - 1][j - 1] + cy;
                if px[i][j] > 0 && px[i][j] == py[i][j] {
                    count += 1;
                }
            }
        }
        count
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
