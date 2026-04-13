// 1848. Minimum Distance to the Target Element
// https://leetcode.com/problems/minimum-distance-to-the-target-element/

struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_distance_to_the_target_element::Solution;

    #[test]
    fn test_get_min_distance_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        let target = 5;
        let start = 3;
        assert_eq!(1, Solution::get_min_distance(nums, target, start));
    }

    #[test]
    fn test_get_min_distance_2() {
        let nums = [1].to_vec();
        let target = 1;
        let start = 0;
        assert_eq!(0, Solution::get_min_distance(nums, target, start));
    }

    #[test]
    fn test_get_min_distance_3() {
        let nums = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1].to_vec();
        let target = 1;
        let start = 0;
        assert_eq!(0, Solution::get_min_distance(nums, target, start));
    }
}
