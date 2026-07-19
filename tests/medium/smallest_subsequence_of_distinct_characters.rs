// 1081. Smallest Subsequence of Distinct Characters
// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/

struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let bytes = s.as_bytes();
        let mut last_index = [0usize; 26];

        for (index, &byte) in bytes.iter().enumerate() {
            last_index[(byte - b'a') as usize] = index;
        }

        let mut stack = Vec::with_capacity(bytes.len().min(26));
        let mut used = [false; 26];

        for (index, &byte) in bytes.iter().enumerate() {
            let char_index = (byte - b'a') as usize;
            if used[char_index] {
                continue;
            }

            while let Some(&last_byte) = stack.last() {
                let last_char_index = (last_byte - b'a') as usize;
                if last_byte <= byte || last_index[last_char_index] <= index {
                    break;
                }

                stack.pop();
                used[last_char_index] = false;
            }

            stack.push(byte);
            used[char_index] = true;
        }

        String::from_utf8(stack).expect("result contains only lowercase ASCII letters")
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_subsequence_of_distinct_characters::Solution;

    #[test]
    fn test_smallest_subsequence_1() {
        let s = "bcabc".to_string();
        assert_eq!("abc".to_string(), Solution::smallest_subsequence(s));
    }

    #[test]
    fn test_smallest_subsequence_2() {
        let s = "cbacdcbc".to_string();
        assert_eq!("acdb".to_string(), Solution::smallest_subsequence(s));
    }
}
