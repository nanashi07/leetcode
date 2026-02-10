// 3719. Longest Balanced Subarray I
// https://leetcode.com/problems/longest-balanced-subarray-i/

struct Solution;

use std::collections::HashMap;

impl Solution {
    // https://leetcode.com/problems/longest-balanced-subarray-i/editorial/
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;

        for i in 0..nums.len() {
            let mut odd = HashMap::new();
            let mut even = HashMap::new();

            for j in i..nums.len() {
                let map = if nums[j] & 1 == 1 {
                    &mut odd
                } else {
                    &mut even
                };

                *map.entry(nums[j]).or_insert(0) += 1;

                if odd.len() == even.len() {
                    max_len = max_len.max((j - i + 1) as i32);
                }
            }
        }

        max_len
    }
}
#[cfg(test)]
mod tests {
    use crate::medium::longest_balanced_subarray_i::Solution;

    #[test]
    fn test_longest_balanced_1() {
        let nums = [2, 5, 4, 3].to_vec();
        assert_eq!(4, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_2() {
        let nums = [3, 2, 2, 5, 4].to_vec();
        assert_eq!(5, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_3() {
        let nums = [1, 2, 3, 2].to_vec();
        assert_eq!(3, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_4() {
        let nums = [8, 5].to_vec();
        assert_eq!(2, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_5() {
        let nums = [6, 2].to_vec();
        assert_eq!(0, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_6() {
        let nums = [5, 3].to_vec();
        assert_eq!(0, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_7() {
        let nums = [1, 3, 2].to_vec();
        assert_eq!(2, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_8() {
        let nums = [2, 7, 7].to_vec();
        assert_eq!(3, Solution::longest_balanced(nums));
    }
}
