// 3498. Reverse Degree of a String
// https://leetcode.com/problems/reverse-degree-of-a-string/

struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut p = 0;
        let a = b'a' as i32 - 1;
        for (i, c) in s.chars().enumerate() {
            let r = 27 - (c as i32 - a);
            p += (i as i32 + 1) * r;
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::reverse_degree_of_a_string::Solution;

    #[test]
    fn test_reverse_degree_1() {
        let s = "abc".to_string();
        assert_eq!(148, Solution::reverse_degree(s));
    }

    #[test]
    fn test_reverse_degree_2() {
        let s = "zaza".to_string();
        assert_eq!(160, Solution::reverse_degree(s));
    }
}
