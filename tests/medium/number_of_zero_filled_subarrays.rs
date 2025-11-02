// 2348. Number of Zero-Filled Subarrays
// https://leetcode.com/problems/number-of-zero-filled-subarrays/

struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        println!("nums: {:?}", &nums);

        let mut len = 0;
        let mut sum = 0;

        // calculate subarrays
        let calc = |l: i64| {
            let mut l = l;
            let mut s = 0;
            while l > 0 {
                s += l;
                l -= 1;
            }
            s
        };

        for i in 0..nums.len() {
            let n = nums[i];

            if n == 0 {
                len += 1;
            } else {
                sum += calc(len);
                len = 0;
            }
        }

        sum + calc(len)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_zero_filled_subarrays::Solution;

    #[test]
    fn test_zero_filled_subarray_1() {
        let nums = [1, 3, 0, 0, 2, 0, 0, 4].to_vec();
        assert_eq!(6, Solution::zero_filled_subarray(nums));
    }

    #[test]
    fn test_zero_filled_subarray_2() {
        let nums = [0, 0, 0, 2, 0, 0].to_vec();
        assert_eq!(9, Solution::zero_filled_subarray(nums));
    }

    #[test]
    fn test_zero_filled_subarray_3() {
        let nums = [2, 10, 2019].to_vec();
        assert_eq!(0, Solution::zero_filled_subarray(nums));
    }
}
