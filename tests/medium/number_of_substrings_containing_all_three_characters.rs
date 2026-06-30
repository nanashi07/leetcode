// 1358. Number of Substrings Containing All Three Characters
// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_substrings_containing_all_three_characters::Solution;

    #[test]
    fn test_number_of_substrings_1() {
        let s = "abcabc".to_string();
        assert_eq!(10, Solution::number_of_substrings(s));
    }

    #[test]
    fn test_number_of_substrings_2() {
        let s = "aaacb".to_string();
        assert_eq!(3, Solution::number_of_substrings(s));
    }

    #[test]
    fn test_number_of_substrings_3() {
        let s = "abc".to_string();
        assert_eq!(1, Solution::number_of_substrings(s));
    }
}
