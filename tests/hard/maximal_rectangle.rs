// 85. Maximal Rectangle
// https://leetcode.com/problems/maximal-rectangle/

struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut heights = vec![0; cols];
        let mut max_area = 0;

        for row in 0..rows {
            // Update heights for current row
            for col in 0..cols {
                if matrix[row][col] == '1' {
                    heights[col] += 1;
                } else {
                    heights[col] = 0;
                }
            }

            // Find largest rectangle in histogram
            max_area = max_area.max(Self::largest_rectangle_in_histogram(&heights));
        }

        max_area
    }

    fn largest_rectangle_in_histogram(heights: &[i32]) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        let n = heights.len();

        for i in 0..n {
            // Pop from stack while current height is less than stack top height
            while !stack.is_empty() && heights[i] < heights[*stack.last().unwrap()] {
                let h_idx = stack.pop().unwrap();
                let height = heights[h_idx];
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    (i - stack.last().unwrap() - 1) as i32
                };
                max_area = max_area.max(height * width);
            }
            stack.push(i);
        }

        // Pop remaining elements from stack
        while !stack.is_empty() {
            let h_idx = stack.pop().unwrap();
            let height = heights[h_idx];
            let width = if stack.is_empty() {
                n as i32
            } else {
                (n - stack.last().unwrap() - 1) as i32
            };
            max_area = max_area.max(height * width);
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximal_rectangle::Solution;

    #[test]
    fn test_maximal_rectangle_1() {
        let matrix = [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ]
        .iter()
        .map(|l| {
            l.iter()
                .map(|s| s.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        assert_eq!(6, Solution::maximal_rectangle(matrix));
    }

    #[test]
    fn test_maximal_rectangle_2() {
        let matrix = [["0"]]
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::maximal_rectangle(matrix));
    }

    #[test]
    fn test_maximal_rectangle_3() {
        let matrix = [["1"]]
            .iter()
            .map(|l| {
                l.iter()
                    .map(|s| s.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::maximal_rectangle(matrix));
    }
}
