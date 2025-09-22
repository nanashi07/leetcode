// # 3005. Count Elements With Maximum Frequency
// https://leetcode.com/problems/count-elements-with-maximum-frequency/

struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_elements_with_maximum_frequency::Solution;

    #[test]
    fn test_max_frequency_elements_1() {
        let nums = [1, 2, 2, 3, 1, 4].to_vec();
        assert_eq!(4, Solution::max_frequency_elements(nums));
    }

    #[test]
    fn test_max_frequency_elements_2() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(5, Solution::max_frequency_elements(nums));
    }
}
