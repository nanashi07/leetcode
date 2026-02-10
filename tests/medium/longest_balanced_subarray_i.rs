// 3719. Longest Balanced Subarray I
// https://leetcode.com/problems/longest-balanced-subarray-i/

struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let n = nums.len();
        let mut max_length = 0;

        for i in 0..n {
            let mut min_val = nums[i];
            let mut max_val = nums[i];
            let mut unique = HashSet::new();
            unique.insert(nums[i]);

            for j in i..n {
                min_val = min_val.min(nums[j]);
                max_val = max_val.max(nums[j]);
                unique.insert(nums[j]);

                let length = (j - i + 1) as i32;
                let unique_count = unique.len() as i32;
                let range_size = max_val - min_val + 1;

                if length == 2 {
                    // For length 2, the difference must be odd
                    if (max_val - min_val) % 2 == 1 {
                        max_length = max_length.max(length);
                    }
                }
                // For length > 2, all values in range must be present
                else if length > 2 && unique_count == range_size {
                    if length == unique_count {
                        // No duplicates
                        max_length = max_length.max(length);
                    } else if length == unique_count + 1 {
                        // Exactly one duplicate - check if consecutive
                        let subarray = &nums[i..=j];
                        let mut has_consecutive_dup = false;
                        for k in 1..subarray.len() {
                            if subarray[k] == subarray[k - 1] {
                                has_consecutive_dup = true;
                                break;
                            }
                        }
                        if has_consecutive_dup {
                            max_length = max_length.max(length);
                        }
                    }
                }
            }
        }

        max_length
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
}
