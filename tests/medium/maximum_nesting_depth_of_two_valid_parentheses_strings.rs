// 1111. Maximum Nesting Depth of Two Valid Parentheses Strings
// https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/

struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_nesting_depth_of_two_valid_parentheses_strings::Solution;

    #[test]
    fn test_max_depth_after_split_1() {
        let seq = "(()())".to_string();
        assert_eq!(
            [0, 1, 1, 1, 1, 0].to_vec(),
            Solution::max_depth_after_split(seq)
        );
    }

    #[test]
    fn test_max_depth_after_split_2() {
        let seq = "()(())()".to_string();
        assert_eq!(
            [0, 0, 0, 1, 1, 0, 1, 1].to_vec(),
            Solution::max_depth_after_split(seq)
        );
    }
}
