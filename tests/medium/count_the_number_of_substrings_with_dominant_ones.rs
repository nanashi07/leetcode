// 3234. Count the Number of Substrings With Dominant Ones
// https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_the_number_of_substrings_with_dominant_ones::Solution;

    #[test]
    fn test_number_of_substrings_1() {
        let s = "00011".to_string();
        assert_eq!(5, Solution::number_of_substrings(s));
    }

    #[test]
    fn test_number_of_substrings_2() {
        let s = "101101".to_string();
        assert_eq!(16, Solution::number_of_substrings(s));
    }
}
