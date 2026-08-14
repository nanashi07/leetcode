// 3090. Maximum Length Substring With Two Occurrences
// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/

struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut counts = [0; 256];
        let mut left = 0;
        let mut max_len = 0;
        for right in 0..s.len() {
            let r_byte = s[right] as usize;
            counts[r_byte] += 1;
            while counts[r_byte] > 2 {
                counts[s[left] as usize] -= 1;
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_length_substring_with_two_occurrences::Solution;

    #[test]
    fn test_maximum_length_substring_1() {
        let s = "bcbbbcba".to_string();
        assert_eq!(4, Solution::maximum_length_substring(s));
    }

    #[test]
    fn test_maximum_length_substring_2() {
        let s = "aaaa".to_string();
        assert_eq!(2, Solution::maximum_length_substring(s));
    }
}
