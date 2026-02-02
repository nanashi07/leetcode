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

        // Key insight: We always start at index 0. From there, we need to pick k-1 more indices.
        // The constraint is that consecutive picked indices differ by at most dist.
        //
        // Strategy: Maintain a sliding window [L, R] where:
        // - L is the smallest index we might pick as our next element
        // - R = L + dist (the furthest we can reach from L)
        // - We track the k-1 smallest elements in this window
        //
        // As we slide L from 1 to n-1, we consider: what if L is the smallest
        // index we pick? Then we need k-2 more from [L+1, min(L+dist, n-1)]...
        // But wait, that's not right either.
        //
        // Actually, the correct interpretation: we slide a window of size dist+1,
        // starting from position 1. At each position, we maintain the k-1 smallest
        // elements in the window [i, min(i+dist, n-1)], representing all indices
        // we could possibly pick if our leftmost choice (after 0) is at position i.

        let mut selected = BTreeSet::new(); // (value, index) - the k-1 smallest in current window
        let mut not_selected = BTreeSet::new(); // remaining elements in window

        // Initialize: window is [1, min(1+dist, n-1)]
        let mut right = (1 + dist).min(n - 1);
        for i in 1..=right {
            let key = (nums[i], i);
            if selected.len() < k - 1 {
                selected.insert(key);
            } else {
                // Check if this element should replace something in selected
                if let Some(&max_item) = selected.iter().next_back() {
                    if key < max_item {
                        selected.remove(&max_item);
                        not_selected.insert(max_item);
                        selected.insert(key);
                    } else {
                        not_selected.insert(key);
                    }
                } else {
                    not_selected.insert(key);
                }
            }
        }

        // Calculate initial sum
        let mut ans = nums[0] as i64;
        for &(val, _) in selected.iter() {
            ans += val as i64;
        }

        // Slide the window: for each starting position left from 1 to n-1
        for left in 1..n {
            // Remove elements that are now out of range (< left)
            if left > 1 {
                let to_remove = left - 1;
                let key = (nums[to_remove], to_remove);

                if selected.remove(&key) {
                    // Removed from selected, replace with smallest from not_selected
                    if let Some(&item) = not_selected.iter().next() {
                        not_selected.remove(&item);
                        selected.insert(item);
                    }
                } else {
                    not_selected.remove(&key);
                }
            }

            // Add new elements that enter the range
            let new_right = (left + dist).min(n - 1);
            for i in (right + 1)..=new_right {
                if i >= n {
                    break;
                }

                let key = (nums[i], i);

                if selected.len() < k - 1 {
                    selected.insert(key);
                } else if let Some(&max_item) = selected.iter().next_back() {
                    if key < max_item {
                        // Replace max in selected
                        selected.remove(&max_item);
                        not_selected.insert(max_item);
                        selected.insert(key);
                    } else {
                        not_selected.insert(key);
                    }
                } else {
                    not_selected.insert(key);
                }
            }

            right = new_right;

            // Check if we can't form k-1 elements anymore
            if right - left + 1 < k - 1 {
                break;
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

    #[test]
    fn test_minimum_cost_5() {
        let nums = [4, 5, 5, 7, 8, 7, 12].to_vec();
        let k = 5;
        let dist = 5;
        assert_eq!(28, Solution::minimum_cost(nums, k, dist));
    }
}
