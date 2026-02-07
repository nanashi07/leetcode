// 1653. Minimum Deletions to Make String Balanced
// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/

struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_deletions_to_make_string_balanced::Solution;

    #[test]
    fn test_minimum_deletions_1() {
        let s = "aababbab".to_string();
        assert_eq!(2, Solution::minimum_deletions(s));
    }

    #[test]
    fn test_minimum_deletions_2() {
        let s = "bbaaaaabb".to_string();
        assert_eq!(2, Solution::minimum_deletions(s));
    }
}
