// 921. Minimum Add to Make Parentheses Valid
// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_add_to_make_parentheses_valid::Solution;

    #[test]
    fn test_min_add_to_make_valid_1() {
        let s = "())".to_string();
        assert_eq!(1, Solution::min_add_to_make_valid(s));
    }

    #[test]
    fn test_min_add_to_make_valid_2() {
        let s = "(((".to_string();
        assert_eq!(3, Solution::min_add_to_make_valid(s));
    }
}
