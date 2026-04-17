// 3761. Minimum Absolute Distance Between Mirror Pairs
// https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs/

struct Solution;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_absolute_distance_between_mirror_pairs::Solution;

    #[test]
    fn test_min_mirror_pair_distance_1() {
        let nums = [12, 21, 45, 33, 54].to_vec();
        assert_eq!(1, Solution::min_mirror_pair_distance(nums));
    }

    #[test]
    fn test_min_mirror_pair_distance_2() {
        let nums = [120, 21].to_vec();
        assert_eq!(1, Solution::min_mirror_pair_distance(nums));
    }

    #[test]
    fn test_min_mirror_pair_distance_3() {
        let nums = [21, 120].to_vec();
        assert_eq!(-1, Solution::min_mirror_pair_distance(nums));
    }
}
