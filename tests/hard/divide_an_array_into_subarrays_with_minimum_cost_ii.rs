// 3013. Divide an Array Into Subarrays With Minimum Cost II
// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii/

struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        use std::collections::BTreeSet;

        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;

        if k == 1 {
            return nums[0] as i64;
        }

        // Key insight: We need to select k-1 indices from [1, n-1] such that
        // when combined with index 0, consecutive selected indices differ by at most dist.
        // This means we maintain a sliding window of width dist+1, and as we slide,
        // we track the k-1 smallest elements in the valid range.

        // The valid range at any point is: for the earliest unpicked index left,
        // we can pick from [left, left+dist]

        let mut selected = BTreeSet::new(); // (value, index) - the k-1 smallest in current window
        let mut not_selected = BTreeSet::new(); // remaining elements in window

        // Initialize: window is [1, min(1+dist, n-1)]
        // We want k-1 smallest from this window
        for i in 1..=((1 + dist).min(n - 1)) {
            if selected.len() < k - 1 {
                selected.insert((nums[i], i));
            } else {
                not_selected.insert((nums[i], i));
            }
        }

        // Balance: ensure selected has smallest elements
        while selected.len() < k - 1 && !not_selected.is_empty() {
            let &item = not_selected.iter().next().unwrap();
            not_selected.remove(&item);
            selected.insert(item);
        }

        // Calculate initial sum
        let mut ans = nums[0] as i64;
        for &(val, _) in selected.iter() {
            ans += val as i64;
        }

        // Slide the window: left boundary moves from 1 to n-k+1
        // As we move left from i to i+1, we:
        // 1. Remove index i from the window
        // 2. Add index i+dist+1 to the window (if it exists)
        for left in 1..n {
            let right = (left + dist).min(n - 1);

            // Can't form k-1 elements
            if right - left + 1 < k - 1 {
                break;
            }

            // Remove elements < left
            if left > 1 {
                let to_remove = left - 1;
                let remove_val = nums[to_remove];
                let key = (remove_val, to_remove);

                if selected.remove(&key) {
                    // Removed from selected, need to replace with smallest from not_selected
                    if let Some(&item) = not_selected.iter().next() {
                        not_selected.remove(&item);
                        selected.insert(item);
                    }
                } else {
                    not_selected.remove(&key);
                }
            }

            // Add new element at right if it's beyond previous right
            let prev_right = ((left - 1) + dist).min(n - 1);
            if right > prev_right && right < n {
                let new_idx = right;
                let new_val = nums[new_idx];
                let new_key = (new_val, new_idx);

                if selected.len() < k - 1 {
                    selected.insert(new_key);
                } else if let Some(&(max_val, max_idx)) = selected.iter().next_back() {
                    if new_val < max_val {
                        // Replace max in selected
                        selected.remove(&(max_val, max_idx));
                        not_selected.insert((max_val, max_idx));
                        selected.insert(new_key);
                    } else {
                        not_selected.insert(new_key);
                    }
                } else {
                    not_selected.insert(new_key);
                }
            }

            // Calculate sum for this configuration
            if selected.len() == k - 1 {
                let mut sum = nums[0] as i64;
                for &(val, _) in selected.iter() {
                    sum += val as i64;
                }
                ans = ans.min(sum);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::divide_an_array_into_subarrays_with_minimum_cost_ii::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let nums = [1, 3, 2, 6, 4, 2].to_vec();
        let k = 3;
        let dist = 3;
        assert_eq!(5, Solution::minimum_cost(nums, k, dist));
    }

    #[test]
    fn test_minimum_cost_2() {
        let nums = [10, 1, 2, 2, 2, 1].to_vec();
        let k = 4;
        let dist = 3;
        assert_eq!(15, Solution::minimum_cost(nums, k, dist));
    }

    #[test]
    fn test_minimum_cost_3() {
        let nums = [10, 8, 18, 9].to_vec();
        let k = 3;
        let dist = 1;
        assert_eq!(36, Solution::minimum_cost(nums, k, dist));
    }

    #[test]
    fn test_minimum_cost_4() {
        let nums = [1, 5, 6, 6, 3, 7, 2].to_vec();
        let k = 6;
        let dist = 5;
        assert_eq!(23, Solution::minimum_cost(nums, k, dist));
    }
}
