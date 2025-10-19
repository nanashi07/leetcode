// # 1625. Lexicographically Smallest String After Applying Operations
// https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/

struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::lexicographically_smallest_string_after_applying_operations::Solution;

    #[test]
    fn test_find_lex_smallest_string_1() {
        let s = "5525".to_string();
        let a = 9;
        let b = 2;
        assert_eq!(
            "2050".to_string(),
            Solution::find_lex_smallest_string(s, a, b)
        );
    }

    #[test]
    fn test_find_lex_smallest_string_2() {
        let s = "74".to_string();
        let a = 5;
        let b = 1;
        assert_eq!(
            "24".to_string(),
            Solution::find_lex_smallest_string(s, a, b)
        );
    }

    #[test]
    fn test_find_lex_smallest_string_3() {
        let s = "0011".to_string();
        let a = 4;
        let b = 2;
        assert_eq!(
            "0011".to_string(),
            Solution::find_lex_smallest_string(s, a, b)
        );
    }
}
