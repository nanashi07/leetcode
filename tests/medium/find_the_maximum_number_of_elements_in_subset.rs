// 3020. Find the Maximum Number of Elements in Subset
// https://leetcode.com/problems/find-the-maximum-number-of-elements-in-subset/

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_maximum_number_of_elements_in_subset::Solution;

    #[test]
    fn test_maximum_length_1() {
        let nums = [5, 4, 1, 2, 2].to_vec();
        assert_eq!(3, Solution::maximum_length(nums));
    }

    #[test]
    fn test_maximum_length_2() {
        let nums = [1, 3, 2, 4].to_vec();
        assert_eq!(1, Solution::maximum_length(nums));
    }
}
