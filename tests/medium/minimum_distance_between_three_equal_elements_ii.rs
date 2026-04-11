// 3741. Minimum Distance Between Three Equal Elements II
// https://leetcode.com/problems/minimum-distance-between-three-equal-elements-ii/

struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_distance_between_three_equal_elements_ii::Solution;

    #[test]
    fn test_minimum_distance_1() {
        let nums = [1, 2, 1, 1, 3].to_vec();
        assert_eq!(6, Solution::minimum_distance(nums));
    }

    #[test]
    fn test_minimum_distance_2() {
        let nums = [1, 1, 2, 3, 2, 1, 2].to_vec();
        assert_eq!(8, Solution::minimum_distance(nums));
    }

    #[test]
    fn test_minimum_distance_3() {
        let nums = [1].to_vec();
        assert_eq!(-1, Solution::minimum_distance(nums));
    }
}
