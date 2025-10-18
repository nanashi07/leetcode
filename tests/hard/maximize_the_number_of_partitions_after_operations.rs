// # 3003. Maximize the Number of Partitions After Operations
// https://leetcode.com/problems/maximize-the-number-of-partitions-after-operations/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let k = k as usize;

        // Memoization: (position, current_mask, changed_used) -> max_partitions
        let mut memo: HashMap<(usize, u32, bool), i32> = HashMap::new();

        fn dp(
            pos: usize,
            cur_mask: u32,
            changed: bool,
            chars: &[char],
            k: usize,
            memo: &mut HashMap<(usize, u32, bool), i32>,
        ) -> i32 {
            // Base case: reached end of string
            if pos == chars.len() {
                return if cur_mask == 0 { 0 } else { 1 };
            }

            // Check memo
            let key = (pos, cur_mask, changed);
            if let Some(&result) = memo.get(&key) {
                return result;
            }

            let char_bit = 1u32 << (chars[pos] as u32 - 'a' as u32);
            let mut result = 0;

            // Option 1: Don't change current character
            let new_mask = cur_mask | char_bit;
            let distinct = new_mask.count_ones() as usize;

            if distinct <= k {
                // Can continue current partition
                result = result.max(dp(pos + 1, new_mask, changed, chars, k, memo));
            } else {
                // Must start new partition
                result = result.max(1 + dp(pos + 1, char_bit, changed, chars, k, memo));
            }

            // Option 2: Change current character (if not already used)
            if !changed {
                // Try changing to each possible character
                for c in 0..26 {
                    let new_char_bit = 1u32 << c;
                    let new_mask_with_change = cur_mask | new_char_bit;
                    let distinct_with_change = new_mask_with_change.count_ones() as usize;

                    if distinct_with_change <= k {
                        // Can continue current partition with changed char
                        result =
                            result.max(dp(pos + 1, new_mask_with_change, true, chars, k, memo));
                    } else {
                        // Must start new partition with changed char
                        result = result.max(1 + dp(pos + 1, new_char_bit, true, chars, k, memo));
                    }
                }
            }

            memo.insert(key, result);
            result
        }

        dp(0, 0, false, &chars, k, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_the_number_of_partitions_after_operations::Solution;

    #[test]
    fn test_max_partitions_after_operations_1() {
        let s = "accca".to_string();
        let k = 2;
        assert_eq!(3, Solution::max_partitions_after_operations(s, k));
    }

    #[test]
    fn test_max_partitions_after_operations_2() {
        let s = "aabaab".to_string();
        let k = 3;
        assert_eq!(1, Solution::max_partitions_after_operations(s, k));
    }

    #[test]
    fn test_max_partitions_after_operations_3() {
        let s = "xxyz".to_string();
        let k = 1;
        assert_eq!(4, Solution::max_partitions_after_operations(s, k));
    }
}
