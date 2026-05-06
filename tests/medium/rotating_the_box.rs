// 1861. Rotating the Box
// https://leetcode.com/problems/rotating-the-box/

struct Solution;

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();
        // Rotated grid: n rows, m cols (90° clockwise)
        let mut result = vec![vec!['.'; m]; n];

        for i in 0..m {
            // Simulate gravity: stones fall to the right before rotation
            let mut empty = n as i32 - 1; // rightmost available position
            for j in (0..n).rev() {
                match box_grid[i][j] {
                    '*' => {
                        // Place obstacle and reset empty pointer
                        result[j][m - 1 - i] = '*';
                        empty = j as i32 - 1;
                    }
                    '#' => {
                        // Drop stone to the rightmost empty slot
                        result[empty as usize][m - 1 - i] = '#';
                        empty -= 1;
                    }
                    _ => {}
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::rotating_the_box::Solution;
    use crate::shared::vec2d::to_char_vec2d;

    #[test]
    fn test_rotate_the_box_1() {
        let box_grid = to_char_vec2d([["#", ".", "#"]]);
        let output = to_char_vec2d([["."], ["#"], ["#"]]);
        assert_eq!(output, Solution::rotate_the_box(box_grid));
    }

    #[test]
    fn test_rotate_the_box_2() {
        let box_grid = to_char_vec2d([["#", ".", "*", "."], ["#", "#", "*", "."]]);
        let output = to_char_vec2d([["#", "."], ["#", "#"], ["*", "*"], [".", "."]]);
        assert_eq!(output, Solution::rotate_the_box(box_grid));
    }

    #[test]
    fn test_rotate_the_box_3() {
        let box_grid = to_char_vec2d([
            ["#", "#", "*", ".", "*", "."],
            ["#", "#", "#", "*", ".", "."],
            ["#", "#", "#", ".", "#", "."],
        ]);
        let output = to_char_vec2d([
            [".", "#", "#"],
            [".", "#", "#"],
            ["#", "#", "*"],
            ["#", "*", "."],
            ["#", ".", "*"],
            ["#", ".", "."],
        ]);
        assert_eq!(output, Solution::rotate_the_box(box_grid));
    }
}
