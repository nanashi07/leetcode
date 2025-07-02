// # 3333. Find the Original Typed String II
// https://leetcode.com/problems/find-the-original-typed-string-ii/

struct Solution;

impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_original_typed_string_ii::Solution;

    #[test]
    fn test_possible_string_count_1() {
        let word = "aabbccdd".to_owned();
        let k = 7;
        assert_eq!(5, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_2() {
        let word = "aabbccdd".to_owned();
        let k = 8;
        assert_eq!(1, Solution::possible_string_count(word, k));
    }

    #[test]
    fn test_possible_string_count_3() {
        let word = "aaabbb".to_owned();
        let k = 3;
        assert_eq!(8, Solution::possible_string_count(word, k));
    }
}
