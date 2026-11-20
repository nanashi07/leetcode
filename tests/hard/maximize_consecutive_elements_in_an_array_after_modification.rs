// 3041. Maximize Consecutive Elements in an Array After Modification
// https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification/

struct Solution;

impl Solution {
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_consecutive_elements_in_an_array_after_modification::Solution;

    #[test]
    fn test_max_selected_elements_1() {
        let nums: Vec<i32> = vec![2, 1, 5, 1, 1];
        assert_eq!(3, Solution::max_selected_elements(nums));
    }

    #[test]
    fn test_max_selected_elements_2() {
        let nums: Vec<i32> = vec![1, 4, 7, 10];
        assert_eq!(1, Solution::max_selected_elements(nums));
    }
}
