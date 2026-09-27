// 1190. Reverse Substrings Between Each Pair of Parentheses
// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/

struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::reverse_substrings_between_each_pair_of_parentheses::Solution;

    #[test]
    fn test_reverse_parentheses_1() {
        let s = "(abcd)".to_string();
        assert_eq!("dcba".to_string(), Solution::reverse_parentheses(s));
    }

    #[test]
    fn test_reverse_parentheses_2() {
        let s = "(u(love)i)".to_string();
        assert_eq!("iloveu".to_string(), Solution::reverse_parentheses(s));
    }

    #[test]
    fn test_reverse_parentheses_3() {
        let s = "(ed(et(oc))el)".to_string();
        assert_eq!("leetcode".to_string(), Solution::reverse_parentheses(s));
    }
}
