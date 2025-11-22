// 3190. Find Minimum Operations to Make All Elements Divisible by Three
// https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_minimum_operations_to_make_all_elements_divisible_by_three::Solution;

    #[test]
    fn test_minimum_operations_1() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(3, Solution::minimum_operations(nums));
    }

    #[test]
    fn test_minimum_operations_2() {
        let nums = [3, 6, 9].to_vec();
        assert_eq!(0, Solution::minimum_operations(nums));
    }
}
