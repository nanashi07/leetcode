// # 120. Triangle
// https://leetcode.com/problems/triangle/

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
            return 0;
        }

        let mut triangle = triangle; // Make it mutable

        // Start from the second-to-last row and work upwards
        for row in (0..triangle.len() - 1).rev() {
            for col in 0..triangle[row].len() {
                // For each position, add the minimum of the two adjacent positions below
                triangle[row][col] += triangle[row + 1][col].min(triangle[row + 1][col + 1]);
            }
        }

        triangle[0][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::triangle::Solution;

    #[test]
    fn test_minimum_total_1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(11, Solution::minimum_total(triangle));
    }

    #[test]
    fn test_minimum_total_2() {
        let triangle = vec![vec![-10]];
        assert_eq!(-10, Solution::minimum_total(triangle));
    }
}
