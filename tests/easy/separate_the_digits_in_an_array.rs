// 2553. Separate the Digits in an Array
// https://leetcode.com/problems/separate-the-digits-in-an-array/

struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::separate_the_digits_in_an_array::Solution;

    #[test]
    fn test_separate_digits_1() {
        let nums = [13, 25, 83, 77].to_vec();
        assert_eq!(
            [1, 3, 2, 5, 8, 3, 7, 7].to_vec(),
            Solution::separate_digits(nums)
        );
    }

    #[test]
    fn test_separate_digits_2() {
        let nums = [7, 1, 3, 9].to_vec();
        assert_eq!([7, 1, 3, 9].to_vec(), Solution::separate_digits(nums));
    }
}
