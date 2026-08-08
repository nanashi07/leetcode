// 3302. Find the Lexicographically Smallest Valid Sequence
// https://leetcode.com/problems/find-the-lexicographically-smallest-valid-sequence/

struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_lexicographically_smallest_valid_sequence::Solution;

    #[test]
    fn test_valid_sequence_1() {
        let word1 = "vbcca".to_string();
        let word2 = "abc".to_string();
        assert_eq!([0, 1, 2].to_vec(), Solution::valid_sequence(word1, word2));
    }

    #[test]
    fn test_valid_sequence_2() {
        let word1 = "bacdc".to_string();
        let word2 = "abc".to_string();
        assert_eq!([1, 2, 4].to_vec(), Solution::valid_sequence(word1, word2));
    }

    #[test]
    fn test_valid_sequence_3() {
        let word1 = "aaaaaa".to_string();
        let word2 = "aaabc".to_string();
        assert_eq!([0; 0].to_vec(), Solution::valid_sequence(word1, word2));
    }

    #[test]
    fn test_valid_sequence_4() {
        let word1 = "abc".to_string();
        let word2 = "ab".to_string();
        assert_eq!([0, 1].to_vec(), Solution::valid_sequence(word1, word2));
    }
}
