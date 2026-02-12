// 3713. Longest Balanced Substring I
// https://leetcode.com/problems/longest-balanced-substring-i/

struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut max_len = 0;

        // Try all possible substrings
        for i in 0..n {
            let mut freq = std::collections::HashMap::new();
            for j in i..n {
                *freq.entry(chars[j]).or_insert(0) += 1;

                // Check if all distinct characters have the same count
                let counts: Vec<i32> = freq.values().cloned().collect();
                if counts.len() > 0 && counts.iter().all(|&c| c == counts[0]) {
                    max_len = max_len.max(j - i + 1);
                }
            }
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_balanced_substring_i::Solution;

    #[test]
    fn test_longest_balanced_1() {
        let s = "abbac".to_string();
        assert_eq!(4, Solution::longest_balanced(s));
    }

    #[test]
    fn test_longest_balanced_2() {
        let s = "zzabccy".to_string();
        assert_eq!(4, Solution::longest_balanced(s));
    }

    #[test]
    fn test_longest_balanced_3() {
        let s = "aba".to_string();
        assert_eq!(2, Solution::longest_balanced(s));
    }
}
