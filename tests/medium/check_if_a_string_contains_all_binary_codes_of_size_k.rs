// 1461. Check If a String Contains All Binary Codes of Size K
// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/

struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        use std::collections::HashSet;

        let k = k as usize;
        let s_bytes = s.as_bytes();

        // Early return if string is too short
        if s_bytes.len() < k {
            return false;
        }

        // We need 2^k different codes
        let required_count = 1 << k; // 2^k

        // Collect all unique substrings of length k
        let mut seen = HashSet::new();

        for i in 0..=s_bytes.len() - k {
            let substring = &s_bytes[i..i + k];
            seen.insert(substring);

            // Early exit if we've found all codes
            if seen.len() == required_count {
                return true;
            }
        }

        seen.len() == required_count
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
