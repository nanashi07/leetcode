// 3020. Find the Maximum Number of Elements in Subset
// https://leetcode.com/problems/find-the-maximum-number-of-elements-in-subset/

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        for &num in &nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut max_len = 0;

        if let Some(&ones_count) = counts.get(&1) {
            max_len = if ones_count % 2 == 1 {
                ones_count
            } else {
                ones_count - 1
            };
        }

        for &x in counts.keys() {
            if x == 1 {
                continue;
            }

            let mut current_len = 0;
            let mut y = x;
            loop {
                let y_count = *counts.get(&y).unwrap_or(&0);
                if y_count >= 2 {
                    if let Some(next_y) = y.checked_mul(y) {
                        if counts.contains_key(&next_y) {
                            current_len += 2;
                            y = next_y;
                            continue;
                        }
                    }
                }
                current_len += 1;
                break;
            }
            if current_len > max_len {
                max_len = current_len;
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_maximum_number_of_elements_in_subset::Solution;

    #[test]
    fn test_maximum_length_1() {
        let nums = [5, 4, 1, 2, 2].to_vec();
        assert_eq!(3, Solution::maximum_length(nums));
    }

    #[test]
    fn test_maximum_length_2() {
        let nums = [1, 3, 2, 4].to_vec();
        assert_eq!(1, Solution::maximum_length(nums));
    }
}
