// 1461. Check If a String Contains All Binary Codes of Size K
// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/

struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::check_if_a_string_contains_all_binary_codes_of_size_k::Solution;

    #[test]
    fn test_has_all_codes_1() {
        let s = "00110110".to_string();
        let k = 2;
        assert_eq!(true, Solution::has_all_codes(s, k));
    }

    #[test]
    fn test_has_all_codes_2() {
        let s = "0110".to_string();
        let k = 1;
        assert_eq!(true, Solution::has_all_codes(s, k));
    }

    #[test]
    fn test_has_all_codes_3() {
        let s = "0110".to_string();
        let k = 2;
        assert_eq!(false, Solution::has_all_codes(s, k));
    }
}
