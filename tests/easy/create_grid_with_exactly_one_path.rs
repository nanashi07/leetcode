// 3963. Create Grid With Exactly One Path
// https://leetcode.com/problems/create-grid-with-exactly-one-path/

struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::create_grid_with_exactly_one_path::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_create_grid_1() {
        let m = 2;
        let n = 3;
        let output = to_string_vec(["..#", "#.."]);
        assert_eq!(output, Solution::create_grid(m, n));
    }

    #[test]
    fn test_create_grid_2() {
        let m = 3;
        let n = 3;
        let output = to_string_vec(["..#", "#..", "##."]);
        assert_eq!(output, Solution::create_grid(m, n));
    }

    #[test]
    fn test_create_grid_3() {
        let m = 1;
        let n = 4;
        let output = to_string_vec(["...."]);
        assert_eq!(output, Solution::create_grid(m, n));
    }
}
