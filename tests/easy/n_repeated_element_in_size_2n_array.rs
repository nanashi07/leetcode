// 961. N-Repeated Element in Size 2N Array
// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/

struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut seen = HashSet::new();

        for num in nums {
            if !seen.insert(num) {
                return num;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::n_repeated_element_in_size_2n_array::Solution;

    #[test]
    fn test_repeated_n_times_1() {
        let nums = [1, 2, 3, 3].to_vec();
        assert_eq!(3, Solution::repeated_n_times(nums));
    }

    #[test]
    fn test_repeated_n_times_2() {
        let nums = [2, 1, 2, 5, 3, 2].to_vec();
        assert_eq!(2, Solution::repeated_n_times(nums));
    }

    #[test]
    fn test_repeated_n_times_3() {
        let nums = [5, 1, 5, 2, 5, 3, 5, 4].to_vec();
        assert_eq!(5, Solution::repeated_n_times(nums));
    }
}
