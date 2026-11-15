// 1524. Number of Sub-arrays With Odd Sum
// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/

struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_sub_arrays_with_odd_sum::Solution;

    #[test]
    fn test_num_of_subarrays_1() {
        let arr: Vec<i32> = vec![1, 3, 5];
        assert_eq!(4, Solution::num_of_subarrays(arr));
    }

    #[test]
    fn test_num_of_subarrays_2() {
        let arr: Vec<i32> = vec![2, 4, 6];
        assert_eq!(0, Solution::num_of_subarrays(arr));
    }

    #[test]
    fn test_num_of_subarrays_3() {
        let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(16, Solution::num_of_subarrays(arr));
    }
}
