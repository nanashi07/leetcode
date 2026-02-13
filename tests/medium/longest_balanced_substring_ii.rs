// 3714. Longest Balanced Substring II
// https://leetcode.com/problems/longest-balanced-substring-ii/

struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_balanced_substring_ii::Solution;

    #[test]
    fn test_longest_balanced_1() {
        let s = "abbac".to_string();
        assert_eq!(4, Solution::longest_balanced(s));
    }

    #[test]
    fn test_longest_balanced_2() {
        let s = "aabcc".to_string();
        assert_eq!(3, Solution::longest_balanced(s));
    }

    #[test]
    fn test_longest_balanced_3() {
        let s = "aba".to_string();
        assert_eq!(2, Solution::longest_balanced(s));
    }
}
