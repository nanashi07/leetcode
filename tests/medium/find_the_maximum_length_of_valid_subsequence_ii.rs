// 3202. Find the Maximum Length of Valid Subsequence II
// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/editorial/
    // Dynamic Programming approach to find maximum length of valid subsequence
    // 使用動態規劃方法找到有效子序列的最大長度
    //
    // The goal is to find the longest subsequence where consecutive elements sum to the same value modulo k
    // 目標是找到最長的子序列，其中連續元素的和對k取模後為相同值
    //
    // Algorithm explanation:
    // 算法說明：
    // 1. Use a 2D DP table where dp[i][j] represents the length of subsequence ending with
    //    an element that has modulo value j, and the previous element has modulo value i
    // 1. 使用二維DP表，dp[i][j]表示以模值為j的元素結尾的子序列長度，
    //    且前一個元素的模值為i
    //
    // 2. For each number, we calculate its modulo value and update all possible transitions
    // 2. 對於每個數字，我們計算其模值並更新所有可能的轉換
    //
    // 3. The transition is: if we add current number (mod k = mod_num) after a number (mod k = prev),
    //    then dp[prev][mod_num] = dp[mod_num][prev] + 1
    // 3. 轉換關係是：如果我們在模值為prev的數字後面添加當前數字（模值為mod_num），
    //    則 dp[prev][mod_num] = dp[mod_num][prev] + 1
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        // Convert k to usize for array indexing
        // 將k轉換為usize以便數組索引
        let k = k as usize;

        // Initialize DP table: dp[i][j] = max length of subsequence where
        // last element mod k = j and second-to-last element mod k = i
        // 初始化DP表：dp[i][j] = 子序列的最大長度，其中
        // 最後元素模k = j，倒數第二個元素模k = i
        let mut dp = vec![vec![0; k]; k];

        // Track the maximum length found so far
        // 追蹤目前找到的最大長度
        let mut res = 0;

        // Process each number in the input array
        // 處理輸入數組中的每個數字
        for num in nums {
            // Calculate the modulo value of current number
            // 計算當前數字的模值
            let mod_num = (num % k as i32) as usize;

            // Try all possible previous modulo values
            // 嘗試所有可能的前一個模值
            for prev in 0..k {
                // Update DP: extend subsequence ending with prev by adding current number
                // The key insight: dp[prev][mod_num] stores length when we go from prev to mod_num
                // And dp[mod_num][prev] stores length when we go from mod_num to prev
                // 更新DP：通過添加當前數字來擴展以prev結尾的子序列
                // 關鍵洞察：dp[prev][mod_num]存儲從prev到mod_num的長度
                // dp[mod_num][prev]存儲從mod_num到prev的長度
                dp[prev][mod_num] = dp[mod_num][prev] + 1;

                // Update the maximum result
                // 更新最大結果
                res = res.max(dp[prev][mod_num]);
            }
        }

        // Return the maximum length found
        // 返回找到的最大長度
        res
    }

    // time exceed
    pub fn _maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        println!("nums: {:?}, k: {}", &nums, k);

        if nums.len() < 3 {
            return nums.len() as i32;
        }

        let mut max = 0;

        for n in 0..k {
            let mut last_i = 0;
            let mut m = 0;
            for i in last_i + 1..nums.len() {
                if (nums[last_i] + nums[i]) % k == n {
                    m += 1;
                    last_i = i;
                }
            }
            max = max.max(m);
        }

        max = max + 1;

        println!("max: {}", max);

        max = max.max(Self::maximum_length(nums[1..].to_vec(), k));

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_maximum_length_of_valid_subsequence_ii::Solution;

    #[test]
    fn test_maximum_length_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        let k = 2;
        assert_eq!(5, Solution::maximum_length(nums, k));
    }

    #[test]
    fn test_maximum_length_2() {
        let nums = [1, 4, 2, 3, 1, 4].to_vec();
        let k = 3;
        assert_eq!(4, Solution::maximum_length(nums, k));
    }

    #[test]
    fn test_maximum_length_3() {
        let nums = [1, 2, 3, 10, 2].to_vec();
        let k = 6;
        assert_eq!(3, Solution::maximum_length(nums, k));
    }
}
