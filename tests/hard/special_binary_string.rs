// 761. Special Binary String
// https://leetcode.com/problems/special-binary-string/

struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::special_binary_string::Solution;

    #[test]
    fn test_make_largest_special_1() {
        let s = "11011000".to_string();
        assert_eq!("11100100".to_string(), Solution::make_largest_special(s));
    }

    #[test]
    fn test_make_largest_special_2() {
        let s = "10".to_string();
        assert_eq!("10".to_string(), Solution::make_largest_special(s));
    }
}
