// 2911. Minimum Changes to Make K Semi-palindromes
// https://leetcode.com/problems/minimum-changes-to-make-k-semi-palindromes/

struct Solution;

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_changes_to_make_k_semi_palindromes::Solution;

    #[test]
    fn test_minimum_changes_1() {
        let s: String = "abcac".to_string();
        let k: i32 = 2;
        assert_eq!(1, Solution::minimum_changes(s, k));
    }

    #[test]
    fn test_minimum_changes_2() {
        let s: String = "abcac".to_string();
        let k: i32 = 2;
        assert_eq!(1, Solution::minimum_changes(s, k));
    }

    #[test]
    fn test_minimum_changes_3() {
        let s: String = "abcac".to_string();
        let k: i32 = 2;
        assert_eq!(1, Solution::minimum_changes(s, k));
    }
}
