// 1861. Rotating the Box
// https://leetcode.com/problems/rotating-the-box/

struct Solution;

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        todo!()
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
