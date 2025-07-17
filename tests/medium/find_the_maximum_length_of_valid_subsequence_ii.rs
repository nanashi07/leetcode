// # 3202. Find the Maximum Length of Valid Subsequence II
// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/editorial/
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        let mut res = 0;
        for num in nums {
            let mod_num = (num % k as i32) as usize;
            for prev in 0..k {
                dp[prev][mod_num] = dp[mod_num][prev] + 1;
                res = res.max(dp[prev][mod_num]);
            }
        }
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
