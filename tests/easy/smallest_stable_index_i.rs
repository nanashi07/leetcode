// 3903. Smallest Stable Index I
// https://leetcode.com/problems/smallest-stable-index-i/

struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        for i in 0..nums.len() {
            let max = *nums[0..=i].iter().max().unwrap();
            let min = *nums[i..nums.len()].iter().min().unwrap();
            let score = max - min;
            if score <= k {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::smallest_stable_index_i::Solution;

    #[test]
    fn test_first_stable_index_1() {
        let nums = [5, 0, 1, 4].to_vec();
        let k = 3;
        assert_eq!(3, Solution::first_stable_index(nums, k));
    }

    #[test]
    fn test_first_stable_index_2() {
        let nums = [3, 2, 1].to_vec();
        let k = 1;
        assert_eq!(-1, Solution::first_stable_index(nums, k));
    }

    #[test]
    fn test_first_stable_index_3() {
        let nums = [0].to_vec();
        let k = 0;
        assert_eq!(0, Solution::first_stable_index(nums, k));
    }
}
