// 2267. Check if There Is a Valid Parentheses String Path
// https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path/

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::check_if_there_is_a_valid_parentheses_string_path::Solution;
    use crate::shared::vec2d::{to_char_vec2d, to_string_vec2d};

    #[test]
    fn test_has_valid_path_1() {
        let grid = to_char_vec2d([
            ["(", "(", "("],
            [")", "(", ")"],
            ["(", "(", ")"],
            ["(", "(", ")"],
        ]);
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_has_valid_path_2() {
        let grid = to_char_vec2d([
            ["(", "(", "("],
            [")", "(", ")"],
            ["(", "(", ")"],
            ["(", "(", ")"],
        ]);
        assert!(Solution::has_valid_path(grid));
    }
}
