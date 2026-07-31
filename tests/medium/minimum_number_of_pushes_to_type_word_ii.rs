// 3016. Minimum Number of Pushes to Type Word II
// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/

struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_pushes_to_type_word_ii::Solution;

    #[test]
    fn test_minimum_pushes_1() {
        let word = "abcde".to_string();
        assert_eq!(5, Solution::minimum_pushes(word));
    }

    #[test]
    fn test_minimum_pushes_2() {
        let word = "xyzxyzxyzxyz".to_string();
        assert_eq!(12, Solution::minimum_pushes(word));
    }

    #[test]
    fn test_minimum_pushes_3() {
        let word = "aabbccddeeffgghhiiiiii".to_string();
        assert_eq!(24, Solution::minimum_pushes(word));
    }
}
