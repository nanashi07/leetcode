// 1477. Find Two Non-overlapping Sub-arrays Each With Target Sum
// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/

struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_two_non_overlapping_sub_arrays_each_with_target_sum::Solution;

    #[test]
    fn test_min_sum_of_lengths_1() {
        let arr = [3, 2, 2, 4, 3].to_vec();
        let target = 3;
        assert_eq!(2, Solution::min_sum_of_lengths(arr, target));
    }

    #[test]
    fn test_min_sum_of_lengths_2() {
        let arr = [7, 3, 4, 7].to_vec();
        let target = 7;
        assert_eq!(2, Solution::min_sum_of_lengths(arr, target));
    }

    #[test]
    fn test_min_sum_of_lengths_3() {
        let arr = [4, 3, 2, 6, 2, 3, 4].to_vec();
        let target = 6;
        assert_eq!(-1, Solution::min_sum_of_lengths(arr, target));
    }
}
