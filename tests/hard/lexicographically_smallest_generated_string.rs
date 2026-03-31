// 3474. Lexicographically Smallest Generated String
// https://leetcode.com/problems/lexicographically-smallest-generated-string/

struct Solution;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::lexicographically_smallest_generated_string::Solution;

    #[test]
    fn test_generate_string_1() {
        let str1 = "TFTF".to_string();
        let str2 = "ab".to_string();
        assert_eq!("ababa".to_string(), Solution::generate_string(str1, str2));
    }

    #[test]
    fn test_generate_string_2() {
        let str1 = "TFTF".to_string();
        let str2 = "abc".to_string();
        assert_eq!("".to_string(), Solution::generate_string(str1, str2));
    }

    #[test]
    fn test_generate_string_3() {
        let str1 = "F".to_string();
        let str2 = "d".to_string();
        assert_eq!("a".to_string(), Solution::generate_string(str1, str2));
    }
}
