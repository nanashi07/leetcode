// 1614. Maximum Nesting Depth of the Parentheses
// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/

struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_nesting_depth_of_the_parentheses::Solution;

    #[test]
    fn test_max_depth_1() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();
        assert_eq!(3, Solution::max_depth(s));
    }

    #[test]
    fn test_max_depth_2() {
        let s = "(1)+((2))+(((3)))".to_string();
        assert_eq!(3, Solution::max_depth(s));
    }

    #[test]
    fn test_max_depth_3() {
        let s = "()(())((()()))".to_string();
        assert_eq!(3, Solution::max_depth(s));
    }
}
