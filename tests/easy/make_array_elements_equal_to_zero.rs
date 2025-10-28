// # 3354. Make Array Elements Equal to Zero
// https://leetcode.com/problems/make-array-elements-equal-to-zero/

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::make_array_elements_equal_to_zero::Solution;

    #[test]
    fn test_count_valid_selections_1() {
        let nums = [1, 0, 2, 0, 3].to_vec();
        assert_eq!(2, Solution::count_valid_selections(nums));
    }

    #[test]
    fn test_count_valid_selections_2() {
        let nums = [2, 3, 4, 0, 4, 1, 0].to_vec();
        assert_eq!(0, Solution::count_valid_selections(nums));
    }
}
