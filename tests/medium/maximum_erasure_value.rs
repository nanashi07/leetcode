// # 1695. Maximum Erasure Value
// https://leetcode.com/problems/maximum-erasure-value/

use std::collections::HashSet;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/maximum-erasure-value/solutions/6988015/from-editorial-two-pointers-with-set-and-boolean-array-beats-100/?envType=daily-question&envId=2025-07-22

    /// Finds the maximum sum of a subarray with all unique elements using sliding window technique
    /// 使用滑動窗口技術找出所有元素都唯一的子陣列的最大和
    ///
    /// Algorithm: Two Pointers / Sliding Window with HashSet
    /// 演算法：雙指針/滑動窗口配合哈希集合
    ///
    /// Time Complexity: O(n) - each element is visited at most twice
    /// 時間複雜度：O(n) - 每個元素最多被訪問兩次
    ///
    /// Space Complexity: O(min(m, n)) where m is the number of unique elements
    /// 空間複雜度：O(min(m, n))，其中 m 是唯一元素的數量
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        // Track the maximum sum found so far
        // 追蹤目前找到的最大和
        let mut max = 0;

        // Track the current sum of the sliding window
        // 追蹤滑動窗口的當前和
        let mut current_sum = 0;

        // Left pointer of the sliding window (start index)
        // 滑動窗口的左指針（起始索引）
        let mut start = 0;

        // HashSet to track unique elements in current window
        // 哈希集合用於追蹤當前窗口中的唯一元素
        let mut set: HashSet<i32> = HashSet::new();

        // Right pointer of the sliding window (end index)
        // 滑動窗口的右指針（結束索引）
        for end in 0..nums.len() {
            // If current element already exists in window, shrink window from left
            // 如果當前元素已存在於窗口中，從左側縮小窗口
            while set.contains(&nums[end]) {
                // Remove the leftmost element from set and subtract from sum
                // 從集合中移除最左邊的元素並從和中減去
                set.remove(&nums[start]);
                current_sum -= nums[start];

                // Move left pointer right to shrink window
                // 將左指針向右移動以縮小窗口
                start += 1;
            }

            // Add current element to window: update sum and set
            // 將當前元素加入窗口：更新和與集合
            current_sum += nums[end];
            set.insert(nums[end]);

            // Update maximum sum if current window sum is larger
            // 如果當前窗口的和更大，則更新最大和
            max = max.max(current_sum);
        }

        max
    }

    // Time Limit Exceeded
    pub fn _maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let mut max = 0;

        for i in 0..nums.len() {
            let mut set: HashSet<i32> = HashSet::new();
            let n = nums[i];
            set.insert(n);
            for j in i + 1..nums.len() {
                let m = nums[j];
                if set.contains(&m) {
                    break;
                } else {
                    set.insert(m);
                }
            }
            max = max.max(set.iter().sum());
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_erasure_value::Solution;

    #[test]
    fn test_maximum_unique_subarray_1() {
        let nums = [4, 2, 4, 5, 6].to_vec();
        assert_eq!(17, Solution::maximum_unique_subarray(nums));
    }

    #[test]
    fn test_maximum_unique_subarray_2() {
        let nums = [5, 2, 1, 2, 5, 2, 1, 2, 5].to_vec();
        assert_eq!(8, Solution::maximum_unique_subarray(nums));
    }
}
