// 3714. Longest Balanced Substring II
// https://leetcode.com/problems/longest-balanced-substring-ii/

struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut max_len = 0;

        // Try all possible substrings
        for i in 0..n {
            let mut freq = [0; 26];

            for j in i..n {
                let idx = (s[j] - b'a') as usize;
                freq[idx] += 1;

                // Check if all characters have the same frequency
                let mut target_freq = 0;
                let mut is_balanced = true;

                for k in 0..26 {
                    if freq[k] > 0 {
                        if target_freq == 0 {
                            target_freq = freq[k];
                        } else if freq[k] != target_freq {
                            is_balanced = false;
                            break;
                        }
                    }
                }

                if is_balanced {
                    let len = j - i + 1;
                    max_len = max_len.max(len);
                }
            }
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_balanced_substring_ii::Solution;

    #[test]
    fn test_longest_balanced_1() {
        let s = "abbac".to_string();
        assert_eq!(4, Solution::longest_balanced(s));
    }

    #[test]
    fn test_longest_balanced_2() {
        let s = "aabcc".to_string();
        assert_eq!(3, Solution::longest_balanced(s));
    }

    #[test]
    fn test_longest_balanced_3() {
        let s = "aba".to_string();
        assert_eq!(2, Solution::longest_balanced(s));
    }
}
