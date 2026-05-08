// 3629. Minimum Jumps to Reach End via Prime Teleportation
// https://leetcode.com/problems/minimum-jumps-to-reach-end-via-prime-teleportation/

struct Solution;

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_jumps_to_reach_end_via_prime_teleportation::Solution;

    #[test]
    fn test_min_jumps_1() {
        let nums = [1, 2, 4, 6].to_vec();
        assert_eq!(2, Solution::min_jumps(nums));
    }

    #[test]
    fn test_min_jumps_2() {
        let nums = [2, 3, 4, 7, 9].to_vec();
        assert_eq!(2, Solution::min_jumps(nums));
    }

    #[test]
    fn test_min_jumps_3() {
        let nums = [4, 6, 5, 8].to_vec();
        assert_eq!(3, Solution::min_jumps(nums));
    }
}
