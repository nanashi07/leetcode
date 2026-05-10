// 2770. Maximum Number of Jumps to Reach the Last Index
// https://leetcode.com/problems/maximum-number-of-jumps-to-reach-the-last-index/

struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_jumps_to_reach_the_last_index::Solution;

    #[test]
    fn test_maximum_jumps_1() {
        let nums = [1, 3, 6, 4, 1, 2].to_vec();
        let target = 2;
        assert_eq!(3, Solution::maximum_jumps(nums, target));
    }

    #[test]
    fn test_maximum_jumps_2() {
        let nums = [1, 3, 6, 4, 1, 2].to_vec();
        let target = 3;
        assert_eq!(5, Solution::maximum_jumps(nums, target));
    }

    #[test]
    fn test_maximum_jumps_3() {
        let nums = [1, 3, 6, 4, 1, 2].to_vec();
        let target = 0;
        assert_eq!(-1, Solution::maximum_jumps(nums, target));
    }
}
