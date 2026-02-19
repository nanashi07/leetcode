// 696. Count Binary Substrings
// https://leetcode.com/problems/count-binary-substrings/

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_binary_substrings::Solution;

    #[test]
    fn test_count_binary_substrings_1() {
        let s = "00110011".to_string();
        assert_eq!(6, Solution::count_binary_substrings(s));
    }

    #[test]
    fn test_count_binary_substrings_2() {
        let s = "10101".to_string();
        assert_eq!(4, Solution::count_binary_substrings(s));
    }
}
