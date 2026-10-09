// 1541. Minimum Insertions to Balance a Parentheses String
// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_insertions_to_balance_a_parentheses_string::Solution;

    #[test]
    fn test_min_insertions_1() {
        let s = "(()))".to_string();
        assert_eq!(1, Solution::min_insertions(s));
    }

    #[test]
    fn test_min_insertions_2() {
        let s = "())".to_string();
        assert_eq!(0, Solution::min_insertions(s));
    }

    #[test]
    fn test_min_insertions_3() {
        let s = "))())(".to_string();
        assert_eq!(3, Solution::min_insertions(s));
    }
}
