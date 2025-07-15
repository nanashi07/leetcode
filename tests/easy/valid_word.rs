// # 3136. Valid Word
// https://leetcode.com/problems/valid-word/

struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::valid_word::Solution;

    #[test]
    fn test_is_valid_1() {
        let word = "234Adas".to_owned();
        assert_eq!(true, Solution::is_valid(word));
    }

    #[test]
    fn test_is_valid_2() {
        let word = "b3".to_owned();
        assert_eq!(false, Solution::is_valid(word));
    }

    #[test]
    fn test_is_valid_3() {
        let word = "a3$e".to_owned();
        assert_eq!(false, Solution::is_valid(word));
    }
}
