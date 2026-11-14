// 3364. Minimum Positive Sum Subarray
// https://leetcode.com/problems/minimum-positive-sum-subarray/

struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let (l, r) = (l as usize, r as usize);
        let n = nums.len();

        let mut prefix = vec![0i32; n + 1];
        for (i, &v) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + v;
        }

        let upper = r.min(n);
        let mut ans: Option<i32> = None;
        for len in l..=upper {
            for end in len..=n {
                let sum = prefix[end] - prefix[end - len];
                if sum > 0 && ans.is_none_or(|cur| sum < cur) {
                    ans = Some(sum);
                }
            }
        }

        ans.unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_positive_sum_subarray::Solution;

    #[test]
    fn test_minimum_sum_subarray_1() {
        let nums: Vec<i32> = vec![3, -2, 1, 4];
        let l: i32 = 2;
        let r: i32 = 3;
        assert_eq!(1, Solution::minimum_sum_subarray(nums, l, r));
    }

    #[test]
    fn test_minimum_sum_subarray_2() {
        let nums: Vec<i32> = vec![-2, 2, -3, 1];
        let l: i32 = 2;
        let r: i32 = 3;
        assert_eq!(-1, Solution::minimum_sum_subarray(nums, l, r));
    }

    #[test]
    fn test_minimum_sum_subarray_3() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let l: i32 = 2;
        let r: i32 = 4;
        assert_eq!(3, Solution::minimum_sum_subarray(nums, l, r));
    }
}
