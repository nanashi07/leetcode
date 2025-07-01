// # 3330. Find the Original Typed String I
// https://leetcode.com/problems/find-the-original-typed-string-i/

struct Solution;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_the_original_typed_string_i::Solution;

    #[test]
    fn test_possible_string_count_1() {
        let word = "abbcccc".to_owned();
        assert_eq!(5, Solution::possible_string_count(word));
    }

    #[test]
    fn test_possible_string_count_2() {
        let word = "abcd".to_owned();
        assert_eq!(1, Solution::possible_string_count(word));
    }

    #[test]
    fn test_possible_string_count_3() {
        let word = "aaaa".to_owned();
        assert_eq!(4, Solution::possible_string_count(word));
    }
}
